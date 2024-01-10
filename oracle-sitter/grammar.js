/*eslint sort-keys: "error"*/

/// <reference path="node_modules/tree-sitter-cli/dsl.d.ts" />
const pluralize = require('pluralize');

/**
 * Returns the upper and lower cased versions of a keyword.
 * @param {string} keyword
 */
function getCases(keyword) {
  const upper = keyword[0].toUpperCase() + keyword.slice(1);
  const lower = keyword[0].toLowerCase() + keyword.slice(1);
  return [upper, lower];
}

/**
 * Returns the singular and plural versions of a keyword.
 * @param {string} keyword
 */
function getQuantities(keyword) {
  return [pluralize.singular(keyword), pluralize.plural(keyword)];
}

/**
 * Creates a rule for upper and lower case versions of a keyword.
 * Keywords may be multi-word tokens, e.g.: "double strike" becomes "Double strike" or "double strike".
 */
const keyword = (string) => choice(...getCases(string));

/**
 * Create a rule matching a type name, like "card", or "land", or "creature", which accepts upper
 * or lowercase first letters, and pluralizes the pattern, e.g.: given sorcery it will accept:
 *   * sorcery
 *   * Sorcery
 *   * sorceries
 *   * Sorceries
 * @param {string} keyword
 */
const typeName = (keyword) => choice(...getQuantities(keyword).flatMap(getQuantities));

const repeat1Sep = (rule, separator) => seq(rule, repeat(seq(separator, rule)));

const keywordChoice = (...strings) => choice(...strings.map(keyword));

const typeNameChoice = (...strings) => choice(...strings.map(typeName));

/*

Tree Sitter Notes

* Regex patterns may not use \b to match word boundaries.

*/
const word = require('tree-sitter').word;

module.exports = grammar({
  name: 'mtg_oracle',

  extras: ($) => [/ /],

  rules: {
    // This is the tree-sitter DSL, but all of the code here must be valid JavaScript. Each key in
    // this map is a function and you must be careful to understand JavaScript's semi-colon
    // insertion rules.

    // This is the special initial rule, it must remain first. It parses a line of rules text.
    rules_line: ($) =>
      choice(
        // TODO: Remove mana ability - it is a special case of an activated ability - all activated abilities that add mana are mana abilities.
        $.mana_ability,
        $.keyword_ability_list,
        $.line_effect,
        $.unblockable_ability,
        $.line_activated_ability,
        $.line_static_ability,
      ),

    keyword_ability_list: ($) => repeat1Sep($.keyword_ability, ','),
    keyword_ability_list: ($) => prec.left(repeat1Sep($.keyword_ability, ',')),

    line_effect: ($) => repeat1(seq($.effect, '.')),
    line_static_ability: ($) =>
      seq(
        $.static_ability_subject,
        optional(seq('have', optional('hexproof from'), $.static_ability_effect)), // Example: "Other creatures get -1/-1."
        // This handles static abilities that affect other creatures or game elements.
      ),

    static_ability_subject: ($) =>
      seq(
        optional($.continuous_tense_qualifier), // "All", "Each", "Every"
        $.subject_nontarget, // Using the existing rule for subjects which are not targeted
      ),

    static_ability_effect: ($) =>
      choice(
        seq('have', $.keyword_ability), // Use the existing keyword_ability rule
        $.protection_qualifier, // Protection, as its own static ability effect
        $.keyword_ability_list, // When multiple static abilities are granted
        // Additionally, new rules can be added if needed to handle other static effects
      ),

    continuous_tense_qualifier: ($) => choice('all', 'each', 'every'),

    draw_effect: ($) =>
      seq(
        $.continuous_tense_qualifier,
        $.player,
        keyword('draws'),
        optional(keyword('a')),
        choice($.number, $.textual_number),
        keyword('card'),
        optional(keyword('s')),
      ),

    effect: ($) => choice($.one_shot_effect, $.draw_effect),
    effect: ($) => choice($.one_shot_effect, $.draw_effect),
    line_activated_ability: ($) =>
      seq(
        $.cost, // Identify the cost of the ability
        ':', // Colon to separate cost from effect
        $.effect, // The effect that occurs as a result of the ability
        '.',
      ),

    keyword_ability: ($) =>
      choice(
        keywordChoice(
          'flying',
          'first strike',
          'lifelink',
          'vigilance',
          'deathtouch',
          'haste',
          'visit',
        ),
        $.keyword_ability_protection,
      ),

    // Definitions for one_shot_effect and its components
    one_shot_effect: ($) => prec(1, seq($.action_verb, choice($.subject, $.subject_list))),

    pt_modifier_effect: ($) =>
      seq($.continuous_tense_qualifier, $.qualifier_type, 'get', $.pt_modifier),
    subject_list: ($) => seq($.subject, repeat(seq('and', $.subject))),
    subject: ($) => choice($.subject_target, $.subject_nontarget),

    destroy_all_subtype: ($) => seq('Destroy', 'all', $.any_subtype),

    action_verb($) {
      return keywordChoice(
        // Using the keywordChoice helper
        'Exile',
        'Destroy',
        'Tap',
        'Untap',
        'Discard',
        'Sacrifice',
        'Create',
        'Counter',
        'Draw',
        'Mill',
        'Scry',
        'Gain',
        'Add',
        'Counter',
        // Some of these don't fit the pattern
      );
    },

    mana_effect: ($) => seq('Add', $.mana_cost),
    mana_ability: ($) => prec.left(1, seq('{T}', ':', 'Add', $.mana_cost)),

    // TODO: Generalize this to handle other types of targets. "Target creature", "Creatures", etc.
    // We likely already have a rule defining the subject of an effect, so we can use that.
    unblockable_ability: ($) => seq('~', "can't be blocked"),

    player: ($) => choice(keyword('player'), keyword('players')),

    subject_target: ($) => {
      return seq(
        optional($.finite_quantity),
        keyword('target'),
        optional($.qualifier_color),
        repeat1($.qualifier), // Zero or many qualifiers like "creature", "with flying"
      );
    },
    tap_colored_creature: ($) =>
      seq($.mana_cost, ':', 'Tap', 'target', $.qualifier_color, 'creature', '.'),

    subject_nontarget: ($) => {
      return seq(
        $.non_finite_quantity, // "all", "each", "every"
        repeat1($.qualifier), // At least one qualifier
      );
    },

    finite_quantity: ($) => choice(seq('up to', $.number), 'another', $.number),

    non_finite_quantity: ($) => choice('all', 'each', 'every', 'any number of'),

    qualifier: ($) =>
      choice(
        '~', // The self-referential symbol, e.g.: the card Walking Dead's oracle text is "{B}: Regenerate ~."
        $.qualifier_color,
        $.qualifier_supertype,
        $.qualifier_type,
        keyword('untapped'),
        keyword('attacking'),
        $.qualifier_control,
        $.subtype,
        $.qualifier_pt,
      ),

    qualifier_control: ($) => choice('you control', 'your control', "opponent's control"),

    mana_color: ($) => choice('W', 'U', 'B', 'R', 'G'),

    // # Symbols inside {} mana costs
    plain_mana_cost_symbol: ($) => $.mana_color,
    generic_mana_cost_symbol: ($) => choice(/[0-9]+/, 'X'), // Generic mana costs like "{0}", "{2}" or "{X}"
    colorless_mana_cost_symbol: ($) => 'C',
    snow_mana_cost_symbol: ($) => 'S',
    hybrid_mana_cost_symbol: ($) => seq(choice($.mana_color, '2'), '/', $.mana_color), // Hybrid mana costs like "{W/U}" or "{2/B}"
    phyrexian_mana_cost_symbol: ($) => seq($.mana_color, '/', 'P'), // Phyrexian mana costs like "{R/P}"
    mana_cost_symbol: ($) =>
      choice(
        $.plain_mana_cost_symbol,
        $.generic_mana_cost_symbol,
        $.colorless_mana_cost_symbol,
        $.snow_mana_cost_symbol,
        $.hybrid_mana_cost_symbol,
        $.phyrexian_mana_cost_symbol,
      ),

    mana_cost: ($) => repeat1(seq('{', $.mana_cost_symbol, '}')), // Any sequence of mana cost symbols inside curly braces.
    action_cost: ($) => seq($.action_verb, $.subject), // Sacrifice a creature, etc.
    cost: ($) =>
      repeat1Sep(
        choice(
          $.mana_cost,
          '{T}', // Tap
          '{Q}', // Untap
          $.action_cost,
          $.cost_life_payment,
          // Add other cost types if necessary and as they become available...
        ),
        ',', // Costs are separated by commas
      ),

    cost_life_payment: ($) =>
      seq(
        'Pay', // The word "Pay"
        $.number, // The amount of life to pay
        'life', // The word "life"
      ),

    qualifier_color: ($) => keywordChoice('white', 'blue', 'black', 'red', 'green', 'colorless'),

    qualifier_supertype: ($) => typeNameChoice('legendary', 'basic', 'snow', 'world'),

    qualifier_type: ($) =>
      typeNameChoice(
        'creature',
        'land',
        'artifact',
        'enchantment',
        'instant',
        'sorcery',
        'planeswalker',
      ),

    subtype: ($) => $.any_subtype,

    modifier: ($) =>
      seq(
        optional($.qualifier_pt),
        choice(keywordChoice('flying', 'hexproof', 'tapped', 'untapped'), $.pt_modifier),
      ),

    qualifier_pt: ($) => seq('with', $.pt_modifier),

    pt_modifier: ($) =>
      seq(
        // Power/Toughness modifiers
        choice('+', '-'),
        $.number,
        '/',
        choice('+', '-'),
        $.number,
      ),

    keyword_ability_protection: ($) => seq(keyword('protection'), 'from', $.protection_qualifier),

    protection_qualifier: ($) =>
      seq(
        choice(
          $.qualifier_color,
          keyword('multicolored'),
          keywordChoice('everything', 'monocolored'), // for protection from everything and monocolored
          $.qualifier_type, // This includes card types like artifacts or enchantments // Additional specific qualifiers if necessary...
        ),
      ),

    number: ($) => /[0-9]+/,
    textual_number: ($) =>
      choice(
        'one',
        'two',
        'three',
        'four',
        'five',
        'six',
        'seven',
        'eight',
        'nine',
        'ten',
        'eleven',
        'twelve',
        'thirteen',
        'fourteen',
        'fifteen',
        'sixteen',
        'seventeen',
        'eighteen',
        'nineteen',
        'twenty',
      ),
    any_subtype: ($) => /[A-Za-z-]+/, // This must remain general - we aren't going to list all 300+ subtypes here. This rule should only be used in the context of a qualifier as the last option in a choice rule.
  },
});

/*

# TODO List:

* Refactor mana ability
* There are a number of duplicate keys in the rules - remove those.

*/
