#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#ifdef _MSC_VER
#pragma optimize("", off)
#elif defined(__clang__)
#pragma clang optimize off
#elif defined(__GNUC__)
#pragma GCC optimize ("O0")
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 131
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 213
#define ALIAS_COUNT 0
#define TOKEN_COUNT 164
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 7
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_COMMA = 1,
  anon_sym_DOT = 2,
  anon_sym_have = 3,
  anon_sym_hexprooffrom = 4,
  anon_sym_all = 5,
  anon_sym_each = 6,
  anon_sym_every = 7,
  anon_sym_Draws = 8,
  anon_sym_draws = 9,
  anon_sym_A = 10,
  anon_sym_a = 11,
  anon_sym_Card = 12,
  anon_sym_card = 13,
  anon_sym_S = 14,
  anon_sym_s = 15,
  anon_sym_COLON = 16,
  anon_sym_Flying = 17,
  anon_sym_flying = 18,
  anon_sym_Firststrike = 19,
  anon_sym_firststrike = 20,
  anon_sym_Lifelink = 21,
  anon_sym_lifelink = 22,
  anon_sym_Vigilance = 23,
  anon_sym_vigilance = 24,
  anon_sym_Deathtouch = 25,
  anon_sym_deathtouch = 26,
  anon_sym_Haste = 27,
  anon_sym_haste = 28,
  anon_sym_Visit = 29,
  anon_sym_visit = 30,
  anon_sym_get = 31,
  anon_sym_Destroy = 32,
  anon_sym_Exile = 33,
  anon_sym_exile = 34,
  anon_sym_destroy = 35,
  anon_sym_Tap = 36,
  anon_sym_tap = 37,
  anon_sym_Untap = 38,
  anon_sym_untap = 39,
  anon_sym_Discard = 40,
  anon_sym_discard = 41,
  anon_sym_Sacrifice = 42,
  anon_sym_sacrifice = 43,
  anon_sym_Create = 44,
  anon_sym_create = 45,
  anon_sym_Counter = 46,
  anon_sym_counter = 47,
  anon_sym_Draw = 48,
  anon_sym_draw = 49,
  anon_sym_Mill = 50,
  anon_sym_mill = 51,
  anon_sym_Scry = 52,
  anon_sym_scry = 53,
  anon_sym_Gain = 54,
  anon_sym_gain = 55,
  anon_sym_Add = 56,
  anon_sym_add = 57,
  anon_sym_LBRACET_RBRACE = 58,
  anon_sym_TILDE = 59,
  anon_sym_can_SQUOTEtbeblocked = 60,
  anon_sym_Player = 61,
  anon_sym_player = 62,
  anon_sym_Players = 63,
  anon_sym_players = 64,
  anon_sym_Target = 65,
  anon_sym_target = 66,
  anon_sym_creature = 67,
  anon_sym_upto = 68,
  anon_sym_another = 69,
  anon_sym_anynumberof = 70,
  anon_sym_Untapped = 71,
  anon_sym_untapped = 72,
  anon_sym_Attacking = 73,
  anon_sym_attacking = 74,
  anon_sym_youcontrol = 75,
  anon_sym_yourcontrol = 76,
  anon_sym_opponent_SQUOTEscontrol = 77,
  anon_sym_W = 78,
  anon_sym_U = 79,
  anon_sym_B = 80,
  anon_sym_R = 81,
  anon_sym_G = 82,
  aux_sym_generic_mana_cost_symbol_token1 = 83,
  anon_sym_X = 84,
  sym_colorless_mana_cost_symbol = 85,
  anon_sym_2 = 86,
  anon_sym_SLASH = 87,
  anon_sym_P = 88,
  anon_sym_LBRACE = 89,
  anon_sym_RBRACE = 90,
  anon_sym_LBRACEQ_RBRACE = 91,
  anon_sym_Pay = 92,
  anon_sym_life = 93,
  anon_sym_White = 94,
  anon_sym_white = 95,
  anon_sym_Blue = 96,
  anon_sym_blue = 97,
  anon_sym_Black = 98,
  anon_sym_black = 99,
  anon_sym_Red = 100,
  anon_sym_red = 101,
  anon_sym_Green = 102,
  anon_sym_green = 103,
  anon_sym_Colorless = 104,
  anon_sym_colorless = 105,
  anon_sym_legendary = 106,
  anon_sym_legendaries = 107,
  anon_sym_basic = 108,
  anon_sym_basics = 109,
  anon_sym_snow = 110,
  anon_sym_snows = 111,
  anon_sym_world = 112,
  anon_sym_worlds = 113,
  anon_sym_creatures = 114,
  anon_sym_land = 115,
  anon_sym_lands = 116,
  anon_sym_artifact = 117,
  anon_sym_artifacts = 118,
  anon_sym_enchantment = 119,
  anon_sym_enchantments = 120,
  anon_sym_instant = 121,
  anon_sym_instants = 122,
  anon_sym_sorcery = 123,
  anon_sym_sorceries = 124,
  anon_sym_planeswalker = 125,
  anon_sym_planeswalkers = 126,
  anon_sym_Hexproof = 127,
  anon_sym_hexproof = 128,
  anon_sym_Tapped = 129,
  anon_sym_tapped = 130,
  anon_sym_with = 131,
  anon_sym_PLUS = 132,
  anon_sym_DASH = 133,
  anon_sym_Protection = 134,
  anon_sym_protection = 135,
  anon_sym_from = 136,
  anon_sym_Multicolored = 137,
  anon_sym_multicolored = 138,
  anon_sym_Everything = 139,
  anon_sym_everything = 140,
  anon_sym_Monocolored = 141,
  anon_sym_monocolored = 142,
  anon_sym_one = 143,
  anon_sym_two = 144,
  anon_sym_three = 145,
  anon_sym_four = 146,
  anon_sym_five = 147,
  anon_sym_six = 148,
  anon_sym_seven = 149,
  anon_sym_eight = 150,
  anon_sym_nine = 151,
  anon_sym_ten = 152,
  anon_sym_eleven = 153,
  anon_sym_twelve = 154,
  anon_sym_thirteen = 155,
  anon_sym_fourteen = 156,
  anon_sym_fifteen = 157,
  anon_sym_sixteen = 158,
  anon_sym_seventeen = 159,
  anon_sym_eighteen = 160,
  anon_sym_nineteen = 161,
  anon_sym_twenty = 162,
  sym_any_subtype = 163,
  sym_rules_line = 164,
  sym_keyword_ability_list = 165,
  sym_line_effect = 166,
  sym_line_static_ability = 167,
  sym_static_ability_subject = 168,
  sym_static_ability_effect = 169,
  sym_continuous_tense_qualifier = 170,
  sym_draw_effect = 171,
  sym_effect = 172,
  sym_line_activated_ability = 173,
  sym_keyword_ability = 174,
  sym_one_shot_effect = 175,
  sym_subject = 176,
  sym_action_verb = 177,
  sym_mana_ability = 178,
  sym_unblockable_ability = 179,
  sym_player = 180,
  sym_subject_target = 181,
  sym_subject_nontarget = 182,
  sym_finite_quantity = 183,
  sym_non_finite_quantity = 184,
  sym_qualifier = 185,
  sym_qualifier_control = 186,
  sym_mana_color = 187,
  sym_plain_mana_cost_symbol = 188,
  sym_generic_mana_cost_symbol = 189,
  sym_snow_mana_cost_symbol = 190,
  sym_hybrid_mana_cost_symbol = 191,
  sym_phyrexian_mana_cost_symbol = 192,
  sym_mana_cost_symbol = 193,
  sym_mana_cost = 194,
  sym_action_cost = 195,
  sym_cost = 196,
  sym_cost_life_payment = 197,
  sym_qualifier_color = 198,
  sym_qualifier_supertype = 199,
  sym_qualifier_type = 200,
  sym_subtype = 201,
  sym_qualifier_pt = 202,
  sym_pt_modifier = 203,
  sym_keyword_ability_protection = 204,
  sym_protection_qualifier = 205,
  sym_number = 206,
  sym_textual_number = 207,
  aux_sym_keyword_ability_list_repeat1 = 208,
  aux_sym_line_effect_repeat1 = 209,
  aux_sym_subject_nontarget_repeat1 = 210,
  aux_sym_mana_cost_repeat1 = 211,
  aux_sym_cost_repeat1 = 212,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_COMMA] = ",",
  [anon_sym_DOT] = ".",
  [anon_sym_have] = "have",
  [anon_sym_hexprooffrom] = "hexproof from",
  [anon_sym_all] = "all",
  [anon_sym_each] = "each",
  [anon_sym_every] = "every",
  [anon_sym_Draws] = "Draws",
  [anon_sym_draws] = "draws",
  [anon_sym_A] = "A",
  [anon_sym_a] = "a",
  [anon_sym_Card] = "Card",
  [anon_sym_card] = "card",
  [anon_sym_S] = "S",
  [anon_sym_s] = "s",
  [anon_sym_COLON] = ":",
  [anon_sym_Flying] = "Flying",
  [anon_sym_flying] = "flying",
  [anon_sym_Firststrike] = "First strike",
  [anon_sym_firststrike] = "first strike",
  [anon_sym_Lifelink] = "Lifelink",
  [anon_sym_lifelink] = "lifelink",
  [anon_sym_Vigilance] = "Vigilance",
  [anon_sym_vigilance] = "vigilance",
  [anon_sym_Deathtouch] = "Deathtouch",
  [anon_sym_deathtouch] = "deathtouch",
  [anon_sym_Haste] = "Haste",
  [anon_sym_haste] = "haste",
  [anon_sym_Visit] = "Visit",
  [anon_sym_visit] = "visit",
  [anon_sym_get] = "get",
  [anon_sym_Destroy] = "Destroy",
  [anon_sym_Exile] = "Exile",
  [anon_sym_exile] = "exile",
  [anon_sym_destroy] = "destroy",
  [anon_sym_Tap] = "Tap",
  [anon_sym_tap] = "tap",
  [anon_sym_Untap] = "Untap",
  [anon_sym_untap] = "untap",
  [anon_sym_Discard] = "Discard",
  [anon_sym_discard] = "discard",
  [anon_sym_Sacrifice] = "Sacrifice",
  [anon_sym_sacrifice] = "sacrifice",
  [anon_sym_Create] = "Create",
  [anon_sym_create] = "create",
  [anon_sym_Counter] = "Counter",
  [anon_sym_counter] = "counter",
  [anon_sym_Draw] = "Draw",
  [anon_sym_draw] = "draw",
  [anon_sym_Mill] = "Mill",
  [anon_sym_mill] = "mill",
  [anon_sym_Scry] = "Scry",
  [anon_sym_scry] = "scry",
  [anon_sym_Gain] = "Gain",
  [anon_sym_gain] = "gain",
  [anon_sym_Add] = "Add",
  [anon_sym_add] = "add",
  [anon_sym_LBRACET_RBRACE] = "{T}",
  [anon_sym_TILDE] = "~",
  [anon_sym_can_SQUOTEtbeblocked] = "can't be blocked",
  [anon_sym_Player] = "Player",
  [anon_sym_player] = "player",
  [anon_sym_Players] = "Players",
  [anon_sym_players] = "players",
  [anon_sym_Target] = "Target",
  [anon_sym_target] = "target",
  [anon_sym_creature] = "creature",
  [anon_sym_upto] = "up to",
  [anon_sym_another] = "another",
  [anon_sym_anynumberof] = "any number of",
  [anon_sym_Untapped] = "Untapped",
  [anon_sym_untapped] = "untapped",
  [anon_sym_Attacking] = "Attacking",
  [anon_sym_attacking] = "attacking",
  [anon_sym_youcontrol] = "you control",
  [anon_sym_yourcontrol] = "your control",
  [anon_sym_opponent_SQUOTEscontrol] = "opponent's control",
  [anon_sym_W] = "W",
  [anon_sym_U] = "U",
  [anon_sym_B] = "B",
  [anon_sym_R] = "R",
  [anon_sym_G] = "G",
  [aux_sym_generic_mana_cost_symbol_token1] = "generic_mana_cost_symbol_token1",
  [anon_sym_X] = "X",
  [sym_colorless_mana_cost_symbol] = "colorless_mana_cost_symbol",
  [anon_sym_2] = "2",
  [anon_sym_SLASH] = "/",
  [anon_sym_P] = "P",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_LBRACEQ_RBRACE] = "{Q}",
  [anon_sym_Pay] = "Pay",
  [anon_sym_life] = "life",
  [anon_sym_White] = "White",
  [anon_sym_white] = "white",
  [anon_sym_Blue] = "Blue",
  [anon_sym_blue] = "blue",
  [anon_sym_Black] = "Black",
  [anon_sym_black] = "black",
  [anon_sym_Red] = "Red",
  [anon_sym_red] = "red",
  [anon_sym_Green] = "Green",
  [anon_sym_green] = "green",
  [anon_sym_Colorless] = "Colorless",
  [anon_sym_colorless] = "colorless",
  [anon_sym_legendary] = "legendary",
  [anon_sym_legendaries] = "legendaries",
  [anon_sym_basic] = "basic",
  [anon_sym_basics] = "basics",
  [anon_sym_snow] = "snow",
  [anon_sym_snows] = "snows",
  [anon_sym_world] = "world",
  [anon_sym_worlds] = "worlds",
  [anon_sym_creatures] = "creatures",
  [anon_sym_land] = "land",
  [anon_sym_lands] = "lands",
  [anon_sym_artifact] = "artifact",
  [anon_sym_artifacts] = "artifacts",
  [anon_sym_enchantment] = "enchantment",
  [anon_sym_enchantments] = "enchantments",
  [anon_sym_instant] = "instant",
  [anon_sym_instants] = "instants",
  [anon_sym_sorcery] = "sorcery",
  [anon_sym_sorceries] = "sorceries",
  [anon_sym_planeswalker] = "planeswalker",
  [anon_sym_planeswalkers] = "planeswalkers",
  [anon_sym_Hexproof] = "Hexproof",
  [anon_sym_hexproof] = "hexproof",
  [anon_sym_Tapped] = "Tapped",
  [anon_sym_tapped] = "tapped",
  [anon_sym_with] = "with",
  [anon_sym_PLUS] = "+",
  [anon_sym_DASH] = "-",
  [anon_sym_Protection] = "Protection",
  [anon_sym_protection] = "protection",
  [anon_sym_from] = "from",
  [anon_sym_Multicolored] = "Multicolored",
  [anon_sym_multicolored] = "multicolored",
  [anon_sym_Everything] = "Everything",
  [anon_sym_everything] = "everything",
  [anon_sym_Monocolored] = "Monocolored",
  [anon_sym_monocolored] = "monocolored",
  [anon_sym_one] = "one",
  [anon_sym_two] = "two",
  [anon_sym_three] = "three",
  [anon_sym_four] = "four",
  [anon_sym_five] = "five",
  [anon_sym_six] = "six",
  [anon_sym_seven] = "seven",
  [anon_sym_eight] = "eight",
  [anon_sym_nine] = "nine",
  [anon_sym_ten] = "ten",
  [anon_sym_eleven] = "eleven",
  [anon_sym_twelve] = "twelve",
  [anon_sym_thirteen] = "thirteen",
  [anon_sym_fourteen] = "fourteen",
  [anon_sym_fifteen] = "fifteen",
  [anon_sym_sixteen] = "sixteen",
  [anon_sym_seventeen] = "seventeen",
  [anon_sym_eighteen] = "eighteen",
  [anon_sym_nineteen] = "nineteen",
  [anon_sym_twenty] = "twenty",
  [sym_any_subtype] = "any_subtype",
  [sym_rules_line] = "rules_line",
  [sym_keyword_ability_list] = "keyword_ability_list",
  [sym_line_effect] = "line_effect",
  [sym_line_static_ability] = "line_static_ability",
  [sym_static_ability_subject] = "static_ability_subject",
  [sym_static_ability_effect] = "static_ability_effect",
  [sym_continuous_tense_qualifier] = "continuous_tense_qualifier",
  [sym_draw_effect] = "draw_effect",
  [sym_effect] = "effect",
  [sym_line_activated_ability] = "line_activated_ability",
  [sym_keyword_ability] = "keyword_ability",
  [sym_one_shot_effect] = "one_shot_effect",
  [sym_subject] = "subject",
  [sym_action_verb] = "action_verb",
  [sym_mana_ability] = "mana_ability",
  [sym_unblockable_ability] = "unblockable_ability",
  [sym_player] = "player",
  [sym_subject_target] = "subject_target",
  [sym_subject_nontarget] = "subject_nontarget",
  [sym_finite_quantity] = "finite_quantity",
  [sym_non_finite_quantity] = "non_finite_quantity",
  [sym_qualifier] = "qualifier",
  [sym_qualifier_control] = "qualifier_control",
  [sym_mana_color] = "mana_color",
  [sym_plain_mana_cost_symbol] = "plain_mana_cost_symbol",
  [sym_generic_mana_cost_symbol] = "generic_mana_cost_symbol",
  [sym_snow_mana_cost_symbol] = "snow_mana_cost_symbol",
  [sym_hybrid_mana_cost_symbol] = "hybrid_mana_cost_symbol",
  [sym_phyrexian_mana_cost_symbol] = "phyrexian_mana_cost_symbol",
  [sym_mana_cost_symbol] = "mana_cost_symbol",
  [sym_mana_cost] = "mana_cost",
  [sym_action_cost] = "action_cost",
  [sym_cost] = "cost",
  [sym_cost_life_payment] = "cost_life_payment",
  [sym_qualifier_color] = "qualifier_color",
  [sym_qualifier_supertype] = "qualifier_supertype",
  [sym_qualifier_type] = "qualifier_type",
  [sym_subtype] = "subtype",
  [sym_qualifier_pt] = "qualifier_pt",
  [sym_pt_modifier] = "pt_modifier",
  [sym_keyword_ability_protection] = "keyword_ability_protection",
  [sym_protection_qualifier] = "protection_qualifier",
  [sym_number] = "number",
  [sym_textual_number] = "textual_number",
  [aux_sym_keyword_ability_list_repeat1] = "keyword_ability_list_repeat1",
  [aux_sym_line_effect_repeat1] = "line_effect_repeat1",
  [aux_sym_subject_nontarget_repeat1] = "subject_nontarget_repeat1",
  [aux_sym_mana_cost_repeat1] = "mana_cost_repeat1",
  [aux_sym_cost_repeat1] = "cost_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_COMMA] = anon_sym_COMMA,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_have] = anon_sym_have,
  [anon_sym_hexprooffrom] = anon_sym_hexprooffrom,
  [anon_sym_all] = anon_sym_all,
  [anon_sym_each] = anon_sym_each,
  [anon_sym_every] = anon_sym_every,
  [anon_sym_Draws] = anon_sym_Draws,
  [anon_sym_draws] = anon_sym_draws,
  [anon_sym_A] = anon_sym_A,
  [anon_sym_a] = anon_sym_a,
  [anon_sym_Card] = anon_sym_Card,
  [anon_sym_card] = anon_sym_card,
  [anon_sym_S] = anon_sym_S,
  [anon_sym_s] = anon_sym_s,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_Flying] = anon_sym_Flying,
  [anon_sym_flying] = anon_sym_flying,
  [anon_sym_Firststrike] = anon_sym_Firststrike,
  [anon_sym_firststrike] = anon_sym_firststrike,
  [anon_sym_Lifelink] = anon_sym_Lifelink,
  [anon_sym_lifelink] = anon_sym_lifelink,
  [anon_sym_Vigilance] = anon_sym_Vigilance,
  [anon_sym_vigilance] = anon_sym_vigilance,
  [anon_sym_Deathtouch] = anon_sym_Deathtouch,
  [anon_sym_deathtouch] = anon_sym_deathtouch,
  [anon_sym_Haste] = anon_sym_Haste,
  [anon_sym_haste] = anon_sym_haste,
  [anon_sym_Visit] = anon_sym_Visit,
  [anon_sym_visit] = anon_sym_visit,
  [anon_sym_get] = anon_sym_get,
  [anon_sym_Destroy] = anon_sym_Destroy,
  [anon_sym_Exile] = anon_sym_Exile,
  [anon_sym_exile] = anon_sym_exile,
  [anon_sym_destroy] = anon_sym_destroy,
  [anon_sym_Tap] = anon_sym_Tap,
  [anon_sym_tap] = anon_sym_tap,
  [anon_sym_Untap] = anon_sym_Untap,
  [anon_sym_untap] = anon_sym_untap,
  [anon_sym_Discard] = anon_sym_Discard,
  [anon_sym_discard] = anon_sym_discard,
  [anon_sym_Sacrifice] = anon_sym_Sacrifice,
  [anon_sym_sacrifice] = anon_sym_sacrifice,
  [anon_sym_Create] = anon_sym_Create,
  [anon_sym_create] = anon_sym_create,
  [anon_sym_Counter] = anon_sym_Counter,
  [anon_sym_counter] = anon_sym_counter,
  [anon_sym_Draw] = anon_sym_Draw,
  [anon_sym_draw] = anon_sym_draw,
  [anon_sym_Mill] = anon_sym_Mill,
  [anon_sym_mill] = anon_sym_mill,
  [anon_sym_Scry] = anon_sym_Scry,
  [anon_sym_scry] = anon_sym_scry,
  [anon_sym_Gain] = anon_sym_Gain,
  [anon_sym_gain] = anon_sym_gain,
  [anon_sym_Add] = anon_sym_Add,
  [anon_sym_add] = anon_sym_add,
  [anon_sym_LBRACET_RBRACE] = anon_sym_LBRACET_RBRACE,
  [anon_sym_TILDE] = anon_sym_TILDE,
  [anon_sym_can_SQUOTEtbeblocked] = anon_sym_can_SQUOTEtbeblocked,
  [anon_sym_Player] = anon_sym_Player,
  [anon_sym_player] = anon_sym_player,
  [anon_sym_Players] = anon_sym_Players,
  [anon_sym_players] = anon_sym_players,
  [anon_sym_Target] = anon_sym_Target,
  [anon_sym_target] = anon_sym_target,
  [anon_sym_creature] = anon_sym_creature,
  [anon_sym_upto] = anon_sym_upto,
  [anon_sym_another] = anon_sym_another,
  [anon_sym_anynumberof] = anon_sym_anynumberof,
  [anon_sym_Untapped] = anon_sym_Untapped,
  [anon_sym_untapped] = anon_sym_untapped,
  [anon_sym_Attacking] = anon_sym_Attacking,
  [anon_sym_attacking] = anon_sym_attacking,
  [anon_sym_youcontrol] = anon_sym_youcontrol,
  [anon_sym_yourcontrol] = anon_sym_yourcontrol,
  [anon_sym_opponent_SQUOTEscontrol] = anon_sym_opponent_SQUOTEscontrol,
  [anon_sym_W] = anon_sym_W,
  [anon_sym_U] = anon_sym_U,
  [anon_sym_B] = anon_sym_B,
  [anon_sym_R] = anon_sym_R,
  [anon_sym_G] = anon_sym_G,
  [aux_sym_generic_mana_cost_symbol_token1] = aux_sym_generic_mana_cost_symbol_token1,
  [anon_sym_X] = anon_sym_X,
  [sym_colorless_mana_cost_symbol] = sym_colorless_mana_cost_symbol,
  [anon_sym_2] = anon_sym_2,
  [anon_sym_SLASH] = anon_sym_SLASH,
  [anon_sym_P] = anon_sym_P,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_LBRACEQ_RBRACE] = anon_sym_LBRACEQ_RBRACE,
  [anon_sym_Pay] = anon_sym_Pay,
  [anon_sym_life] = anon_sym_life,
  [anon_sym_White] = anon_sym_White,
  [anon_sym_white] = anon_sym_white,
  [anon_sym_Blue] = anon_sym_Blue,
  [anon_sym_blue] = anon_sym_blue,
  [anon_sym_Black] = anon_sym_Black,
  [anon_sym_black] = anon_sym_black,
  [anon_sym_Red] = anon_sym_Red,
  [anon_sym_red] = anon_sym_red,
  [anon_sym_Green] = anon_sym_Green,
  [anon_sym_green] = anon_sym_green,
  [anon_sym_Colorless] = anon_sym_Colorless,
  [anon_sym_colorless] = anon_sym_colorless,
  [anon_sym_legendary] = anon_sym_legendary,
  [anon_sym_legendaries] = anon_sym_legendaries,
  [anon_sym_basic] = anon_sym_basic,
  [anon_sym_basics] = anon_sym_basics,
  [anon_sym_snow] = anon_sym_snow,
  [anon_sym_snows] = anon_sym_snows,
  [anon_sym_world] = anon_sym_world,
  [anon_sym_worlds] = anon_sym_worlds,
  [anon_sym_creatures] = anon_sym_creatures,
  [anon_sym_land] = anon_sym_land,
  [anon_sym_lands] = anon_sym_lands,
  [anon_sym_artifact] = anon_sym_artifact,
  [anon_sym_artifacts] = anon_sym_artifacts,
  [anon_sym_enchantment] = anon_sym_enchantment,
  [anon_sym_enchantments] = anon_sym_enchantments,
  [anon_sym_instant] = anon_sym_instant,
  [anon_sym_instants] = anon_sym_instants,
  [anon_sym_sorcery] = anon_sym_sorcery,
  [anon_sym_sorceries] = anon_sym_sorceries,
  [anon_sym_planeswalker] = anon_sym_planeswalker,
  [anon_sym_planeswalkers] = anon_sym_planeswalkers,
  [anon_sym_Hexproof] = anon_sym_Hexproof,
  [anon_sym_hexproof] = anon_sym_hexproof,
  [anon_sym_Tapped] = anon_sym_Tapped,
  [anon_sym_tapped] = anon_sym_tapped,
  [anon_sym_with] = anon_sym_with,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_Protection] = anon_sym_Protection,
  [anon_sym_protection] = anon_sym_protection,
  [anon_sym_from] = anon_sym_from,
  [anon_sym_Multicolored] = anon_sym_Multicolored,
  [anon_sym_multicolored] = anon_sym_multicolored,
  [anon_sym_Everything] = anon_sym_Everything,
  [anon_sym_everything] = anon_sym_everything,
  [anon_sym_Monocolored] = anon_sym_Monocolored,
  [anon_sym_monocolored] = anon_sym_monocolored,
  [anon_sym_one] = anon_sym_one,
  [anon_sym_two] = anon_sym_two,
  [anon_sym_three] = anon_sym_three,
  [anon_sym_four] = anon_sym_four,
  [anon_sym_five] = anon_sym_five,
  [anon_sym_six] = anon_sym_six,
  [anon_sym_seven] = anon_sym_seven,
  [anon_sym_eight] = anon_sym_eight,
  [anon_sym_nine] = anon_sym_nine,
  [anon_sym_ten] = anon_sym_ten,
  [anon_sym_eleven] = anon_sym_eleven,
  [anon_sym_twelve] = anon_sym_twelve,
  [anon_sym_thirteen] = anon_sym_thirteen,
  [anon_sym_fourteen] = anon_sym_fourteen,
  [anon_sym_fifteen] = anon_sym_fifteen,
  [anon_sym_sixteen] = anon_sym_sixteen,
  [anon_sym_seventeen] = anon_sym_seventeen,
  [anon_sym_eighteen] = anon_sym_eighteen,
  [anon_sym_nineteen] = anon_sym_nineteen,
  [anon_sym_twenty] = anon_sym_twenty,
  [sym_any_subtype] = sym_any_subtype,
  [sym_rules_line] = sym_rules_line,
  [sym_keyword_ability_list] = sym_keyword_ability_list,
  [sym_line_effect] = sym_line_effect,
  [sym_line_static_ability] = sym_line_static_ability,
  [sym_static_ability_subject] = sym_static_ability_subject,
  [sym_static_ability_effect] = sym_static_ability_effect,
  [sym_continuous_tense_qualifier] = sym_continuous_tense_qualifier,
  [sym_draw_effect] = sym_draw_effect,
  [sym_effect] = sym_effect,
  [sym_line_activated_ability] = sym_line_activated_ability,
  [sym_keyword_ability] = sym_keyword_ability,
  [sym_one_shot_effect] = sym_one_shot_effect,
  [sym_subject] = sym_subject,
  [sym_action_verb] = sym_action_verb,
  [sym_mana_ability] = sym_mana_ability,
  [sym_unblockable_ability] = sym_unblockable_ability,
  [sym_player] = sym_player,
  [sym_subject_target] = sym_subject_target,
  [sym_subject_nontarget] = sym_subject_nontarget,
  [sym_finite_quantity] = sym_finite_quantity,
  [sym_non_finite_quantity] = sym_non_finite_quantity,
  [sym_qualifier] = sym_qualifier,
  [sym_qualifier_control] = sym_qualifier_control,
  [sym_mana_color] = sym_mana_color,
  [sym_plain_mana_cost_symbol] = sym_plain_mana_cost_symbol,
  [sym_generic_mana_cost_symbol] = sym_generic_mana_cost_symbol,
  [sym_snow_mana_cost_symbol] = sym_snow_mana_cost_symbol,
  [sym_hybrid_mana_cost_symbol] = sym_hybrid_mana_cost_symbol,
  [sym_phyrexian_mana_cost_symbol] = sym_phyrexian_mana_cost_symbol,
  [sym_mana_cost_symbol] = sym_mana_cost_symbol,
  [sym_mana_cost] = sym_mana_cost,
  [sym_action_cost] = sym_action_cost,
  [sym_cost] = sym_cost,
  [sym_cost_life_payment] = sym_cost_life_payment,
  [sym_qualifier_color] = sym_qualifier_color,
  [sym_qualifier_supertype] = sym_qualifier_supertype,
  [sym_qualifier_type] = sym_qualifier_type,
  [sym_subtype] = sym_subtype,
  [sym_qualifier_pt] = sym_qualifier_pt,
  [sym_pt_modifier] = sym_pt_modifier,
  [sym_keyword_ability_protection] = sym_keyword_ability_protection,
  [sym_protection_qualifier] = sym_protection_qualifier,
  [sym_number] = sym_number,
  [sym_textual_number] = sym_textual_number,
  [aux_sym_keyword_ability_list_repeat1] = aux_sym_keyword_ability_list_repeat1,
  [aux_sym_line_effect_repeat1] = aux_sym_line_effect_repeat1,
  [aux_sym_subject_nontarget_repeat1] = aux_sym_subject_nontarget_repeat1,
  [aux_sym_mana_cost_repeat1] = aux_sym_mana_cost_repeat1,
  [aux_sym_cost_repeat1] = aux_sym_cost_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_COMMA] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_have] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_hexprooffrom] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_all] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_each] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_every] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Draws] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_draws] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_A] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_a] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Card] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_card] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_S] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_s] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Flying] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_flying] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Firststrike] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_firststrike] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Lifelink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_lifelink] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Vigilance] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_vigilance] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Deathtouch] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_deathtouch] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Haste] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_haste] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Visit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_visit] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_get] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Destroy] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Exile] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_exile] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_destroy] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Tap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Untap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_untap] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Discard] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_discard] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Sacrifice] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sacrifice] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Create] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_create] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Counter] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_counter] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Draw] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_draw] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Mill] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_mill] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Scry] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_scry] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Gain] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_gain] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Add] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_add] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACET_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_TILDE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_can_SQUOTEtbeblocked] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Player] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_player] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Players] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_players] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Target] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_target] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_creature] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_upto] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_another] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_anynumberof] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Untapped] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_untapped] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Attacking] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_attacking] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_youcontrol] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_yourcontrol] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_opponent_SQUOTEscontrol] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_W] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_U] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_B] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_R] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_G] = {
    .visible = true,
    .named = false,
  },
  [aux_sym_generic_mana_cost_symbol_token1] = {
    .visible = false,
    .named = false,
  },
  [anon_sym_X] = {
    .visible = true,
    .named = false,
  },
  [sym_colorless_mana_cost_symbol] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_2] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_SLASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_P] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACEQ_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Pay] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_life] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_White] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_white] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Blue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_blue] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Black] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_black] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Red] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_red] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Green] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_green] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Colorless] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_colorless] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_legendary] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_legendaries] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_basic] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_basics] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_snow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_snows] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_world] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_worlds] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_creatures] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_land] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_lands] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_artifact] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_artifacts] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enchantment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enchantments] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_instant] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_instants] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sorcery] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sorceries] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_planeswalker] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_planeswalkers] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Hexproof] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_hexproof] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Tapped] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_tapped] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_with] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Protection] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_protection] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_from] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Multicolored] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_multicolored] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Everything] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_everything] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_Monocolored] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_monocolored] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_one] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_two] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_three] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_four] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_five] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_six] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_seven] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_eight] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nine] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_ten] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_eleven] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_twelve] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_thirteen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fourteen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_fifteen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_sixteen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_seventeen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_eighteen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_nineteen] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_twenty] = {
    .visible = true,
    .named = false,
  },
  [sym_any_subtype] = {
    .visible = true,
    .named = true,
  },
  [sym_rules_line] = {
    .visible = true,
    .named = true,
  },
  [sym_keyword_ability_list] = {
    .visible = true,
    .named = true,
  },
  [sym_line_effect] = {
    .visible = true,
    .named = true,
  },
  [sym_line_static_ability] = {
    .visible = true,
    .named = true,
  },
  [sym_static_ability_subject] = {
    .visible = true,
    .named = true,
  },
  [sym_static_ability_effect] = {
    .visible = true,
    .named = true,
  },
  [sym_continuous_tense_qualifier] = {
    .visible = true,
    .named = true,
  },
  [sym_draw_effect] = {
    .visible = true,
    .named = true,
  },
  [sym_effect] = {
    .visible = true,
    .named = true,
  },
  [sym_line_activated_ability] = {
    .visible = true,
    .named = true,
  },
  [sym_keyword_ability] = {
    .visible = true,
    .named = true,
  },
  [sym_one_shot_effect] = {
    .visible = true,
    .named = true,
  },
  [sym_subject] = {
    .visible = true,
    .named = true,
  },
  [sym_action_verb] = {
    .visible = true,
    .named = true,
  },
  [sym_mana_ability] = {
    .visible = true,
    .named = true,
  },
  [sym_unblockable_ability] = {
    .visible = true,
    .named = true,
  },
  [sym_player] = {
    .visible = true,
    .named = true,
  },
  [sym_subject_target] = {
    .visible = true,
    .named = true,
  },
  [sym_subject_nontarget] = {
    .visible = true,
    .named = true,
  },
  [sym_finite_quantity] = {
    .visible = true,
    .named = true,
  },
  [sym_non_finite_quantity] = {
    .visible = true,
    .named = true,
  },
  [sym_qualifier] = {
    .visible = true,
    .named = true,
  },
  [sym_qualifier_control] = {
    .visible = true,
    .named = true,
  },
  [sym_mana_color] = {
    .visible = true,
    .named = true,
  },
  [sym_plain_mana_cost_symbol] = {
    .visible = true,
    .named = true,
  },
  [sym_generic_mana_cost_symbol] = {
    .visible = true,
    .named = true,
  },
  [sym_snow_mana_cost_symbol] = {
    .visible = true,
    .named = true,
  },
  [sym_hybrid_mana_cost_symbol] = {
    .visible = true,
    .named = true,
  },
  [sym_phyrexian_mana_cost_symbol] = {
    .visible = true,
    .named = true,
  },
  [sym_mana_cost_symbol] = {
    .visible = true,
    .named = true,
  },
  [sym_mana_cost] = {
    .visible = true,
    .named = true,
  },
  [sym_action_cost] = {
    .visible = true,
    .named = true,
  },
  [sym_cost] = {
    .visible = true,
    .named = true,
  },
  [sym_cost_life_payment] = {
    .visible = true,
    .named = true,
  },
  [sym_qualifier_color] = {
    .visible = true,
    .named = true,
  },
  [sym_qualifier_supertype] = {
    .visible = true,
    .named = true,
  },
  [sym_qualifier_type] = {
    .visible = true,
    .named = true,
  },
  [sym_subtype] = {
    .visible = true,
    .named = true,
  },
  [sym_qualifier_pt] = {
    .visible = true,
    .named = true,
  },
  [sym_pt_modifier] = {
    .visible = true,
    .named = true,
  },
  [sym_keyword_ability_protection] = {
    .visible = true,
    .named = true,
  },
  [sym_protection_qualifier] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_textual_number] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_keyword_ability_list_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_line_effect_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_subject_nontarget_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_mana_cost_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_cost_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 5,
  [7] = 4,
  [8] = 8,
  [9] = 9,
  [10] = 9,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 11,
  [19] = 17,
  [20] = 14,
  [21] = 21,
  [22] = 22,
  [23] = 13,
  [24] = 16,
  [25] = 21,
  [26] = 12,
  [27] = 15,
  [28] = 22,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 38,
  [39] = 39,
  [40] = 40,
  [41] = 41,
  [42] = 42,
  [43] = 43,
  [44] = 44,
  [45] = 45,
  [46] = 46,
  [47] = 12,
  [48] = 48,
  [49] = 49,
  [50] = 50,
  [51] = 51,
  [52] = 52,
  [53] = 53,
  [54] = 54,
  [55] = 55,
  [56] = 56,
  [57] = 57,
  [58] = 58,
  [59] = 59,
  [60] = 60,
  [61] = 61,
  [62] = 62,
  [63] = 58,
  [64] = 64,
  [65] = 65,
  [66] = 66,
  [67] = 67,
  [68] = 68,
  [69] = 69,
  [70] = 70,
  [71] = 71,
  [72] = 72,
  [73] = 73,
  [74] = 74,
  [75] = 75,
  [76] = 76,
  [77] = 77,
  [78] = 78,
  [79] = 79,
  [80] = 80,
  [81] = 81,
  [82] = 82,
  [83] = 73,
  [84] = 84,
  [85] = 85,
  [86] = 86,
  [87] = 84,
  [88] = 88,
  [89] = 89,
  [90] = 90,
  [91] = 91,
  [92] = 92,
  [93] = 93,
  [94] = 94,
  [95] = 90,
  [96] = 96,
  [97] = 97,
  [98] = 98,
  [99] = 99,
  [100] = 100,
  [101] = 101,
  [102] = 102,
  [103] = 103,
  [104] = 104,
  [105] = 105,
  [106] = 106,
  [107] = 107,
  [108] = 108,
  [109] = 109,
  [110] = 110,
  [111] = 111,
  [112] = 112,
  [113] = 113,
  [114] = 114,
  [115] = 115,
  [116] = 17,
  [117] = 117,
  [118] = 118,
  [119] = 119,
  [120] = 120,
  [121] = 121,
  [122] = 122,
  [123] = 123,
  [124] = 124,
  [125] = 125,
  [126] = 126,
  [127] = 127,
  [128] = 128,
  [129] = 102,
  [130] = 130,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(638);
      if (lookahead == ' ') SKIP(0)
      if (lookahead == '+') ADVANCE(833);
      if (lookahead == ',') ADVANCE(639);
      if (lookahead == '-') ADVANCE(834);
      if (lookahead == '.') ADVANCE(640);
      if (lookahead == '/') ADVANCE(752);
      if (lookahead == '2') ADVANCE(751);
      if (lookahead == ':') ADVANCE(662);
      if (lookahead == 'A') ADVANCE(653);
      if (lookahead == 'B') ADVANCE(742);
      if (lookahead == 'C') ADVANCE(750);
      if (lookahead == 'D') ADVANCE(133);
      if (lookahead == 'E') ADVANCE(604);
      if (lookahead == 'F') ADVANCE(275);
      if (lookahead == 'G') ADVANCE(746);
      if (lookahead == 'H') ADVANCE(31);
      if (lookahead == 'L') ADVANCE(267);
      if (lookahead == 'M') ADVANCE(269);
      if (lookahead == 'P') ADVANCE(754);
      if (lookahead == 'R') ADVANCE(744);
      if (lookahead == 'S') ADVANCE(659);
      if (lookahead == 'T') ADVANCE(24);
      if (lookahead == 'U') ADVANCE(740);
      if (lookahead == 'V') ADVANCE(270);
      if (lookahead == 'W') ADVANCE(738);
      if (lookahead == 'X') ADVANCE(748);
      if (lookahead == 'a') ADVANCE(655);
      if (lookahead == 'b') ADVANCE(39);
      if (lookahead == 'c') ADVANCE(25);
      if (lookahead == 'd') ADVANCE(224);
      if (lookahead == 'e') ADVANCE(33);
      if (lookahead == 'f') ADVANCE(273);
      if (lookahead == 'g') ADVANCE(67);
      if (lookahead == 'h') ADVANCE(26);
      if (lookahead == 'i') ADVANCE(381);
      if (lookahead == 'l') ADVANCE(37);
      if (lookahead == 'm') ADVANCE(299);
      if (lookahead == 'n') ADVANCE(278);
      if (lookahead == 'o') ADVANCE(387);
      if (lookahead == 'p') ADVANCE(329);
      if (lookahead == 'r') ADVANCE(164);
      if (lookahead == 's') ADVANCE(661);
      if (lookahead == 't') ADVANCE(35);
      if (lookahead == 'u') ADVANCE(401);
      if (lookahead == 'v') ADVANCE(307);
      if (lookahead == 'w') ADVANCE(265);
      if (lookahead == 'y') ADVANCE(414);
      if (lookahead == '{') ADVANCE(755);
      if (lookahead == '}') ADVANCE(756);
      if (lookahead == '~') ADVANCE(711);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(747);
      END_STATE();
    case 1:
      if (lookahead == ' ') ADVANCE(82);
      END_STATE();
    case 2:
      if (lookahead == ' ') SKIP(2)
      if (lookahead == 'B') ADVANCE(322);
      if (lookahead == 'C') ADVANCE(440);
      if (lookahead == 'D') ADVANCE(196);
      if (lookahead == 'E') ADVANCE(603);
      if (lookahead == 'F') ADVANCE(275);
      if (lookahead == 'G') ADVANCE(480);
      if (lookahead == 'H') ADVANCE(30);
      if (lookahead == 'L') ADVANCE(267);
      if (lookahead == 'M') ADVANCE(413);
      if (lookahead == 'P') ADVANCE(477);
      if (lookahead == 'R') ADVANCE(163);
      if (lookahead == 'V') ADVANCE(270);
      if (lookahead == 'W') ADVANCE(257);
      if (lookahead == 'a') ADVANCE(478);
      if (lookahead == 'b') ADVANCE(342);
      if (lookahead == 'c') ADVANCE(452);
      if (lookahead == 'd') ADVANCE(228);
      if (lookahead == 'e') ADVANCE(378);
      if (lookahead == 'f') ADVANCE(310);
      if (lookahead == 'g') ADVANCE(514);
      if (lookahead == 'h') ADVANCE(27);
      if (lookahead == 'i') ADVANCE(381);
      if (lookahead == 'l') ADVANCE(38);
      if (lookahead == 'm') ADVANCE(456);
      if (lookahead == 'p') ADVANCE(340);
      if (lookahead == 'r') ADVANCE(164);
      if (lookahead == 's') ADVANCE(418);
      if (lookahead == 'v') ADVANCE(307);
      if (lookahead == 'w') ADVANCE(264);
      END_STATE();
    case 3:
      if (lookahead == ' ') SKIP(3)
      if (lookahead == ',') ADVANCE(639);
      if (lookahead == '.') ADVANCE(640);
      if (lookahead == ':') ADVANCE(662);
      if (lookahead == 'A') ADVANCE(1028);
      if (lookahead == 'B') ADVANCE(954);
      if (lookahead == 'C') ADVANCE(988);
      if (lookahead == 'G') ADVANCE(1004);
      if (lookahead == 'R') ADVANCE(903);
      if (lookahead == 'U') ADVANCE(984);
      if (lookahead == 'W') ADVANCE(935);
      if (lookahead == 'a') ADVANCE(1006);
      if (lookahead == 'b') ADVANCE(868);
      if (lookahead == 'c') ADVANCE(992);
      if (lookahead == 'e') ADVANCE(968);
      if (lookahead == 'g') ADVANCE(1012);
      if (lookahead == 'i') ADVANCE(971);
      if (lookahead == 'l') ADVANCE(874);
      if (lookahead == 'o') ADVANCE(994);
      if (lookahead == 'p') ADVANCE(955);
      if (lookahead == 'r') ADVANCE(910);
      if (lookahead == 's') ADVANCE(973);
      if (lookahead == 'u') ADVANCE(985);
      if (lookahead == 'w') ADVANCE(939);
      if (lookahead == 'y') ADVANCE(986);
      if (lookahead == '~') ADVANCE(711);
      if (lookahead == '-' ||
          ('D' <= lookahead && lookahead <= 'Z') ||
          ('d' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 4:
      if (lookahead == ' ') SKIP(4)
      if (lookahead == 'A') ADVANCE(1028);
      if (lookahead == 'B') ADVANCE(954);
      if (lookahead == 'C') ADVANCE(988);
      if (lookahead == 'G') ADVANCE(1004);
      if (lookahead == 'P') ADVANCE(961);
      if (lookahead == 'R') ADVANCE(903);
      if (lookahead == 'U') ADVANCE(984);
      if (lookahead == 'W') ADVANCE(935);
      if (lookahead == 'a') ADVANCE(960);
      if (lookahead == 'b') ADVANCE(868);
      if (lookahead == 'c') ADVANCE(992);
      if (lookahead == 'e') ADVANCE(883);
      if (lookahead == 'g') ADVANCE(1012);
      if (lookahead == 'i') ADVANCE(971);
      if (lookahead == 'l') ADVANCE(874);
      if (lookahead == 'o') ADVANCE(994);
      if (lookahead == 'p') ADVANCE(962);
      if (lookahead == 'r') ADVANCE(910);
      if (lookahead == 's') ADVANCE(973);
      if (lookahead == 'u') ADVANCE(985);
      if (lookahead == 'w') ADVANCE(939);
      if (lookahead == 'y') ADVANCE(986);
      if (lookahead == '~') ADVANCE(711);
      if (lookahead == '-' ||
          ('D' <= lookahead && lookahead <= 'Z') ||
          ('d' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 5:
      if (lookahead == ' ') SKIP(5)
      if (lookahead == '/') ADVANCE(752);
      if (lookahead == 'A') ADVANCE(652);
      if (lookahead == 'B') ADVANCE(741);
      if (lookahead == 'C') ADVANCE(21);
      if (lookahead == 'G') ADVANCE(745);
      if (lookahead == 'P') ADVANCE(753);
      if (lookahead == 'R') ADVANCE(743);
      if (lookahead == 'T') ADVANCE(59);
      if (lookahead == 'U') ADVANCE(739);
      if (lookahead == 'W') ADVANCE(737);
      if (lookahead == 'a') ADVANCE(654);
      if (lookahead == 'c') ADVANCE(69);
      if (lookahead == 'e') ADVANCE(271);
      if (lookahead == 'f') ADVANCE(274);
      if (lookahead == 'l') ADVANCE(303);
      if (lookahead == 'n') ADVANCE(278);
      if (lookahead == 'o') ADVANCE(386);
      if (lookahead == 's') ADVANCE(135);
      if (lookahead == 't') ADVANCE(77);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(747);
      END_STATE();
    case 6:
      if (lookahead == ' ') SKIP(6)
      if (lookahead == '.') ADVANCE(640);
      if (lookahead == '2') ADVANCE(751);
      if (lookahead == 'B') ADVANCE(741);
      if (lookahead == 'C') ADVANCE(749);
      if (lookahead == 'G') ADVANCE(745);
      if (lookahead == 'R') ADVANCE(743);
      if (lookahead == 'S') ADVANCE(658);
      if (lookahead == 'U') ADVANCE(739);
      if (lookahead == 'W') ADVANCE(737);
      if (lookahead == 'X') ADVANCE(748);
      if (lookahead == 's') ADVANCE(660);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(747);
      END_STATE();
    case 7:
      if (lookahead == ' ') SKIP(7)
      if (lookahead == 'B') ADVANCE(954);
      if (lookahead == 'C') ADVANCE(988);
      if (lookahead == 'G') ADVANCE(1004);
      if (lookahead == 'R') ADVANCE(903);
      if (lookahead == 'W') ADVANCE(935);
      if (lookahead == 'b') ADVANCE(964);
      if (lookahead == 'c') ADVANCE(993);
      if (lookahead == 'g') ADVANCE(1012);
      if (lookahead == 'r') ADVANCE(910);
      if (lookahead == 'w') ADVANCE(940);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 8:
      if (lookahead == ' ') SKIP(8)
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 9:
      if (lookahead == ' ') ADVANCE(81);
      END_STATE();
    case 10:
      if (lookahead == ' ') ADVANCE(238);
      END_STATE();
    case 11:
      if (lookahead == ' ') ADVANCE(559);
      END_STATE();
    case 12:
      if (lookahead == ' ') ADVANCE(421);
      END_STATE();
    case 13:
      if (lookahead == ' ') ADVANCE(385);
      END_STATE();
    case 14:
      if (lookahead == ' ') ADVANCE(95);
      if (lookahead == 'r') ADVANCE(17);
      END_STATE();
    case 15:
      if (lookahead == ' ') ADVANCE(532);
      END_STATE();
    case 16:
      if (lookahead == ' ') ADVANCE(534);
      END_STATE();
    case 17:
      if (lookahead == ' ') ADVANCE(104);
      END_STATE();
    case 18:
      if (lookahead == ' ') ADVANCE(106);
      END_STATE();
    case 19:
      if (lookahead == '\'') ADVANCE(538);
      END_STATE();
    case 20:
      if (lookahead == '\'') ADVANCE(553);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(482);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(482);
      if (lookahead == 'o') ADVANCE(597);
      if (lookahead == 'r') ADVANCE(167);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(89);
      if (lookahead == 'c') ADVANCE(481);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(457);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(361);
      if (lookahead == 'o') ADVANCE(354);
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(536);
      if (lookahead == 'e') ADVANCE(619);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(536);
      if (lookahead == 'e') ADVANCE(620);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(608);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(395);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(529);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(529);
      if (lookahead == 'e') ADVANCE(617);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(621);
      if (lookahead == 'l') ADVANCE(40);
      if (lookahead == 'r') ADVANCE(439);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(84);
      if (lookahead == 'i') ADVANCE(247);
      if (lookahead == 'l') ADVANCE(221);
      if (lookahead == 'n') ADVANCE(88);
      if (lookahead == 'v') ADVANCE(212);
      if (lookahead == 'x') ADVANCE(300);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(84);
      if (lookahead == 'i') ADVANCE(247);
      if (lookahead == 'l') ADVANCE(221);
      if (lookahead == 'v') ADVANCE(212);
      if (lookahead == 'x') ADVANCE(300);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(458);
      if (lookahead == 'e') ADVANCE(362);
      if (lookahead == 'h') ADVANCE(283);
      if (lookahead == 'w') ADVANCE(137);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(609);
      END_STATE();
    case 37:
      if (lookahead == 'a') ADVANCE(390);
      if (lookahead == 'e') ADVANCE(248);
      if (lookahead == 'i') ADVANCE(237);
      END_STATE();
    case 38:
      if (lookahead == 'a') ADVANCE(390);
      if (lookahead == 'i') ADVANCE(242);
      END_STATE();
    case 39:
      if (lookahead == 'a') ADVANCE(527);
      if (lookahead == 'l') ADVANCE(50);
      END_STATE();
    case 40:
      if (lookahead == 'a') ADVANCE(628);
      END_STATE();
    case 41:
      if (lookahead == 'a') ADVANCE(461);
      END_STATE();
    case 42:
      if (lookahead == 'a') ADVANCE(459);
      END_STATE();
    case 43:
      if (lookahead == 'a') ADVANCE(85);
      if (lookahead == 'u') ADVANCE(138);
      END_STATE();
    case 44:
      if (lookahead == 'a') ADVANCE(462);
      if (lookahead == 'e') ADVANCE(362);
      if (lookahead == 'h') ADVANCE(283);
      if (lookahead == 'w') ADVANCE(137);
      END_STATE();
    case 45:
      if (lookahead == 'a') ADVANCE(460);
      END_STATE();
    case 46:
      if (lookahead == 'a') ADVANCE(579);
      END_STATE();
    case 47:
      if (lookahead == 'a') ADVANCE(579);
      if (lookahead == 's') ADVANCE(562);
      END_STATE();
    case 48:
      if (lookahead == 'a') ADVANCE(463);
      END_STATE();
    case 49:
      if (lookahead == 'a') ADVANCE(611);
      END_STATE();
    case 50:
      if (lookahead == 'a') ADVANCE(87);
      if (lookahead == 'u') ADVANCE(139);
      END_STATE();
    case 51:
      if (lookahead == 'a') ADVANCE(464);
      END_STATE();
    case 52:
      if (lookahead == 'a') ADVANCE(612);
      END_STATE();
    case 53:
      if (lookahead == 'a') ADVANCE(613);
      END_STATE();
    case 54:
      if (lookahead == 'a') ADVANCE(333);
      END_STATE();
    case 55:
      if (lookahead == 'a') ADVANCE(542);
      END_STATE();
    case 56:
      if (lookahead == 'a') ADVANCE(402);
      END_STATE();
    case 57:
      if (lookahead == 'a') ADVANCE(479);
      END_STATE();
    case 58:
      if (lookahead == 'a') ADVANCE(578);
      END_STATE();
    case 59:
      if (lookahead == 'a') ADVANCE(501);
      END_STATE();
    case 60:
      if (lookahead == 'a') ADVANCE(493);
      END_STATE();
    case 61:
      if (lookahead == 'a') ADVANCE(281);
      END_STATE();
    case 62:
      if (lookahead == 'a') ADVANCE(396);
      END_STATE();
    case 63:
      if (lookahead == 'a') ADVANCE(614);
      END_STATE();
    case 64:
      if (lookahead == 'a') ADVANCE(97);
      END_STATE();
    case 65:
      if (lookahead == 'a') ADVANCE(494);
      END_STATE();
    case 66:
      if (lookahead == 'a') ADVANCE(282);
      END_STATE();
    case 67:
      if (lookahead == 'a') ADVANCE(282);
      if (lookahead == 'e') ADVANCE(540);
      if (lookahead == 'r') ADVANCE(209);
      END_STATE();
    case 68:
      if (lookahead == 'a') ADVANCE(397);
      END_STATE();
    case 69:
      if (lookahead == 'a') ADVANCE(484);
      END_STATE();
    case 70:
      if (lookahead == 'a') ADVANCE(484);
      if (lookahead == 'o') ADVANCE(602);
      if (lookahead == 'r') ADVANCE(222);
      END_STATE();
    case 71:
      if (lookahead == 'a') ADVANCE(394);
      END_STATE();
    case 72:
      if (lookahead == 'a') ADVANCE(574);
      END_STATE();
    case 73:
      if (lookahead == 'a') ADVANCE(575);
      END_STATE();
    case 74:
      if (lookahead == 'a') ADVANCE(535);
      END_STATE();
    case 75:
      if (lookahead == 'a') ADVANCE(630);
      END_STATE();
    case 76:
      if (lookahead == 'a') ADVANCE(405);
      END_STATE();
    case 77:
      if (lookahead == 'a') ADVANCE(506);
      if (lookahead == 'e') ADVANCE(362);
      if (lookahead == 'h') ADVANCE(283);
      if (lookahead == 'w') ADVANCE(137);
      END_STATE();
    case 78:
      if (lookahead == 'a') ADVANCE(107);
      if (lookahead == 'c') ADVANCE(483);
      if (lookahead == 'e') ADVANCE(606);
      if (lookahead == 'i') ADVANCE(616);
      END_STATE();
    case 79:
      if (lookahead == 'a') ADVANCE(593);
      END_STATE();
    case 80:
      if (lookahead == 'a') ADVANCE(593);
      if (lookahead == 's') ADVANCE(581);
      END_STATE();
    case 81:
      if (lookahead == 'b') ADVANCE(345);
      END_STATE();
    case 82:
      if (lookahead == 'b') ADVANCE(166);
      END_STATE();
    case 83:
      if (lookahead == 'b') ADVANCE(205);
      END_STATE();
    case 84:
      if (lookahead == 'c') ADVANCE(253);
      END_STATE();
    case 85:
      if (lookahead == 'c') ADVANCE(314);
      END_STATE();
    case 86:
      if (lookahead == 'c') ADVANCE(789);
      END_STATE();
    case 87:
      if (lookahead == 'c') ADVANCE(315);
      END_STATE();
    case 88:
      if (lookahead == 'c') ADVANCE(258);
      END_STATE();
    case 89:
      if (lookahead == 'c') ADVANCE(505);
      END_STATE();
    case 90:
      if (lookahead == 'c') ADVANCE(255);
      END_STATE();
    case 91:
      if (lookahead == 'c') ADVANCE(256);
      END_STATE();
    case 92:
      if (lookahead == 'c') ADVANCE(321);
      END_STATE();
    case 93:
      if (lookahead == 'c') ADVANCE(60);
      END_STATE();
    case 94:
      if (lookahead == 'c') ADVANCE(427);
      END_STATE();
    case 95:
      if (lookahead == 'c') ADVANCE(442);
      END_STATE();
    case 96:
      if (lookahead == 'c') ADVANCE(556);
      END_STATE();
    case 97:
      if (lookahead == 'c') ADVANCE(548);
      END_STATE();
    case 98:
      if (lookahead == 'c') ADVANCE(156);
      END_STATE();
    case 99:
      if (lookahead == 'c') ADVANCE(157);
      END_STATE();
    case 100:
      if (lookahead == 'c') ADVANCE(158);
      END_STATE();
    case 101:
      if (lookahead == 'c') ADVANCE(159);
      END_STATE();
    case 102:
      if (lookahead == 'c') ADVANCE(183);
      END_STATE();
    case 103:
      if (lookahead == 'c') ADVANCE(65);
      END_STATE();
    case 104:
      if (lookahead == 'c') ADVANCE(443);
      END_STATE();
    case 105:
      if (lookahead == 'c') ADVANCE(587);
      END_STATE();
    case 106:
      if (lookahead == 'c') ADVANCE(444);
      END_STATE();
    case 107:
      if (lookahead == 'c') ADVANCE(516);
      END_STATE();
    case 108:
      if (lookahead == 'c') ADVANCE(450);
      END_STATE();
    case 109:
      if (lookahead == 'c') ADVANCE(453);
      END_STATE();
    case 110:
      if (lookahead == 'c') ADVANCE(454);
      END_STATE();
    case 111:
      if (lookahead == 'd') ADVANCE(112);
      END_STATE();
    case 112:
      if (lookahead == 'd') ADVANCE(708);
      END_STATE();
    case 113:
      if (lookahead == 'd') ADVANCE(773);
      END_STATE();
    case 114:
      if (lookahead == 'd') ADVANCE(709);
      END_STATE();
    case 115:
      if (lookahead == 'd') ADVANCE(775);
      END_STATE();
    case 116:
      if (lookahead == 'd') ADVANCE(656);
      END_STATE();
    case 117:
      if (lookahead == 'd') ADVANCE(657);
      END_STATE();
    case 118:
      if (lookahead == 'd') ADVANCE(803);
      END_STATE();
    case 119:
      if (lookahead == 'd') ADVANCE(797);
      END_STATE();
    case 120:
      if (lookahead == 'd') ADVANCE(829);
      END_STATE();
    case 121:
      if (lookahead == 'd') ADVANCE(830);
      END_STATE();
    case 122:
      if (lookahead == 'd') ADVANCE(690);
      END_STATE();
    case 123:
      if (lookahead == 'd') ADVANCE(691);
      END_STATE();
    case 124:
      if (lookahead == 'd') ADVANCE(728);
      END_STATE();
    case 125:
      if (lookahead == 'd') ADVANCE(730);
      END_STATE();
    case 126:
      if (lookahead == 'd') ADVANCE(842);
      END_STATE();
    case 127:
      if (lookahead == 'd') ADVANCE(843);
      END_STATE();
    case 128:
      if (lookahead == 'd') ADVANCE(838);
      END_STATE();
    case 129:
      if (lookahead == 'd') ADVANCE(839);
      END_STATE();
    case 130:
      if (lookahead == 'd') ADVANCE(712);
      END_STATE();
    case 131:
      if (lookahead == 'd') ADVANCE(114);
      if (lookahead == 'l') ADVANCE(323);
      if (lookahead == 'n') ADVANCE(441);
      END_STATE();
    case 132:
      if (lookahead == 'd') ADVANCE(57);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(47);
      if (lookahead == 'i') ADVANCE(526);
      if (lookahead == 'r') ADVANCE(28);
      END_STATE();
    case 134:
      if (lookahead == 'e') ADVANCE(47);
      if (lookahead == 'i') ADVANCE(526);
      if (lookahead == 'r') ADVANCE(49);
      END_STATE();
    case 135:
      if (lookahead == 'e') ADVANCE(606);
      if (lookahead == 'i') ADVANCE(616);
      END_STATE();
    case 136:
      if (lookahead == 'e') ADVANCE(844);
      END_STATE();
    case 137:
      if (lookahead == 'e') ADVANCE(343);
      if (lookahead == 'o') ADVANCE(845);
      END_STATE();
    case 138:
      if (lookahead == 'e') ADVANCE(765);
      END_STATE();
    case 139:
      if (lookahead == 'e') ADVANCE(767);
      END_STATE();
    case 140:
      if (lookahead == 'e') ADVANCE(848);
      END_STATE();
    case 141:
      if (lookahead == 'e') ADVANCE(641);
      END_STATE();
    case 142:
      if (lookahead == 'e') ADVANCE(760);
      END_STATE();
    case 143:
      if (lookahead == 'e') ADVANCE(852);
      END_STATE();
    case 144:
      if (lookahead == 'e') ADVANCE(679);
      END_STATE();
    case 145:
      if (lookahead == 'e') ADVANCE(673);
      END_STATE();
    case 146:
      if (lookahead == 'e') ADVANCE(761);
      END_STATE();
    case 147:
      if (lookahead == 'e') ADVANCE(680);
      END_STATE();
    case 148:
      if (lookahead == 'e') ADVANCE(674);
      END_STATE();
    case 149:
      if (lookahead == 'e') ADVANCE(846);
      END_STATE();
    case 150:
      if (lookahead == 'e') ADVANCE(763);
      END_STATE();
    case 151:
      if (lookahead == 'e') ADVANCE(694);
      END_STATE();
    case 152:
      if (lookahead == 'e') ADVANCE(695);
      END_STATE();
    case 153:
      if (lookahead == 'e') ADVANCE(695);
      if (lookahead == 'u') ADVANCE(499);
      END_STATE();
    case 154:
      if (lookahead == 'e') ADVANCE(855);
      END_STATE();
    case 155:
      if (lookahead == 'e') ADVANCE(723);
      END_STATE();
    case 156:
      if (lookahead == 'e') ADVANCE(692);
      END_STATE();
    case 157:
      if (lookahead == 'e') ADVANCE(669);
      END_STATE();
    case 158:
      if (lookahead == 'e') ADVANCE(693);
      END_STATE();
    case 159:
      if (lookahead == 'e') ADVANCE(670);
      END_STATE();
    case 160:
      if (lookahead == 'e') ADVANCE(665);
      END_STATE();
    case 161:
      if (lookahead == 'e') ADVANCE(666);
      END_STATE();
    case 162:
      if (lookahead == 'e') ADVANCE(759);
      END_STATE();
    case 163:
      if (lookahead == 'e') ADVANCE(113);
      END_STATE();
    case 164:
      if (lookahead == 'e') ADVANCE(115);
      END_STATE();
    case 165:
      if (lookahead == 'e') ADVANCE(525);
      END_STATE();
    case 166:
      if (lookahead == 'e') ADVANCE(9);
      END_STATE();
    case 167:
      if (lookahead == 'e') ADVANCE(72);
      END_STATE();
    case 168:
      if (lookahead == 'e') ADVANCE(528);
      END_STATE();
    case 169:
      if (lookahead == 'e') ADVANCE(173);
      END_STATE();
    case 170:
      if (lookahead == 'e') ADVANCE(347);
      END_STATE();
    case 171:
      if (lookahead == 'e') ADVANCE(120);
      END_STATE();
    case 172:
      if (lookahead == 'e') ADVANCE(523);
      END_STATE();
    case 173:
      if (lookahead == 'e') ADVANCE(365);
      END_STATE();
    case 174:
      if (lookahead == 'e') ADVANCE(96);
      END_STATE();
    case 175:
      if (lookahead == 'e') ADVANCE(121);
      END_STATE();
    case 176:
      if (lookahead == 'e') ADVANCE(524);
      END_STATE();
    case 177:
      if (lookahead == 'e') ADVANCE(366);
      END_STATE();
    case 178:
      if (lookahead == 'e') ADVANCE(470);
      END_STATE();
    case 179:
      if (lookahead == 'e') ADVANCE(367);
      END_STATE();
    case 180:
      if (lookahead == 'e') ADVANCE(471);
      END_STATE();
    case 181:
      if (lookahead == 'e') ADVANCE(368);
      END_STATE();
    case 182:
      if (lookahead == 'e') ADVANCE(124);
      END_STATE();
    case 183:
      if (lookahead == 'e') ADVANCE(472);
      END_STATE();
    case 184:
      if (lookahead == 'e') ADVANCE(125);
      END_STATE();
    case 185:
      if (lookahead == 'e') ADVANCE(473);
      END_STATE();
    case 186:
      if (lookahead == 'e') ADVANCE(126);
      END_STATE();
    case 187:
      if (lookahead == 'e') ADVANCE(474);
      END_STATE();
    case 188:
      if (lookahead == 'e') ADVANCE(369);
      END_STATE();
    case 189:
      if (lookahead == 'e') ADVANCE(127);
      END_STATE();
    case 190:
      if (lookahead == 'e') ADVANCE(128);
      END_STATE();
    case 191:
      if (lookahead == 'e') ADVANCE(370);
      END_STATE();
    case 192:
      if (lookahead == 'e') ADVANCE(129);
      END_STATE();
    case 193:
      if (lookahead == 'e') ADVANCE(371);
      END_STATE();
    case 194:
      if (lookahead == 'e') ADVANCE(130);
      END_STATE();
    case 195:
      if (lookahead == 'e') ADVANCE(545);
      END_STATE();
    case 196:
      if (lookahead == 'e') ADVANCE(46);
      if (lookahead == 'r') ADVANCE(53);
      END_STATE();
    case 197:
      if (lookahead == 'e') ADVANCE(372);
      END_STATE();
    case 198:
      if (lookahead == 'e') ADVANCE(546);
      END_STATE();
    case 199:
      if (lookahead == 'e') ADVANCE(373);
      END_STATE();
    case 200:
      if (lookahead == 'e') ADVANCE(374);
      END_STATE();
    case 201:
      if (lookahead == 'e') ADVANCE(149);
      END_STATE();
    case 202:
      if (lookahead == 'e') ADVANCE(475);
      END_STATE();
    case 203:
      if (lookahead == 'e') ADVANCE(375);
      END_STATE();
    case 204:
      if (lookahead == 'e') ADVANCE(476);
      END_STATE();
    case 205:
      if (lookahead == 'e') ADVANCE(489);
      END_STATE();
    case 206:
      if (lookahead == 'e') ADVANCE(391);
      END_STATE();
    case 207:
      if (lookahead == 'e') ADVANCE(55);
      END_STATE();
    case 208:
      if (lookahead == 'e') ADVANCE(530);
      END_STATE();
    case 209:
      if (lookahead == 'e') ADVANCE(177);
      END_STATE();
    case 210:
      if (lookahead == 'e') ADVANCE(485);
      END_STATE();
    case 211:
      if (lookahead == 'e') ADVANCE(58);
      END_STATE();
    case 212:
      if (lookahead == 'e') ADVANCE(486);
      END_STATE();
    case 213:
      if (lookahead == 'e') ADVANCE(398);
      END_STATE();
    case 214:
      if (lookahead == 'e') ADVANCE(188);
      END_STATE();
    case 215:
      if (lookahead == 'e') ADVANCE(191);
      END_STATE();
    case 216:
      if (lookahead == 'e') ADVANCE(400);
      END_STATE();
    case 217:
      if (lookahead == 'e') ADVANCE(197);
      END_STATE();
    case 218:
      if (lookahead == 'e') ADVANCE(199);
      END_STATE();
    case 219:
      if (lookahead == 'e') ADVANCE(200);
      END_STATE();
    case 220:
      if (lookahead == 'e') ADVANCE(203);
      END_STATE();
    case 221:
      if (lookahead == 'e') ADVANCE(607);
      END_STATE();
    case 222:
      if (lookahead == 'e') ADVANCE(73);
      END_STATE();
    case 223:
      if (lookahead == 'e') ADVANCE(348);
      END_STATE();
    case 224:
      if (lookahead == 'e') ADVANCE(80);
      if (lookahead == 'i') ADVANCE(539);
      if (lookahead == 'r') ADVANCE(36);
      END_STATE();
    case 225:
      if (lookahead == 'e') ADVANCE(80);
      if (lookahead == 'i') ADVANCE(539);
      if (lookahead == 'r') ADVANCE(52);
      END_STATE();
    case 226:
      if (lookahead == 'e') ADVANCE(105);
      END_STATE();
    case 227:
      if (lookahead == 'e') ADVANCE(517);
      END_STATE();
    case 228:
      if (lookahead == 'e') ADVANCE(79);
      if (lookahead == 'r') ADVANCE(63);
      END_STATE();
    case 229:
      if (lookahead == 'f') ADVANCE(827);
      END_STATE();
    case 230:
      if (lookahead == 'f') ADVANCE(828);
      END_STATE();
    case 231:
      if (lookahead == 'f') ADVANCE(727);
      END_STATE();
    case 232:
      if (lookahead == 'f') ADVANCE(10);
      END_STATE();
    case 233:
      if (lookahead == 'f') ADVANCE(170);
      END_STATE();
    case 234:
      if (lookahead == 'f') ADVANCE(590);
      if (lookahead == 'r') ADVANCE(537);
      if (lookahead == 'v') ADVANCE(140);
      END_STATE();
    case 235:
      if (lookahead == 'f') ADVANCE(590);
      if (lookahead == 'v') ADVANCE(140);
      END_STATE();
    case 236:
      if (lookahead == 'f') ADVANCE(64);
      END_STATE();
    case 237:
      if (lookahead == 'f') ADVANCE(142);
      END_STATE();
    case 238:
      if (lookahead == 'f') ADVANCE(497);
      END_STATE();
    case 239:
      if (lookahead == 'f') ADVANCE(162);
      END_STATE();
    case 240:
      if (lookahead == 'f') ADVANCE(288);
      END_STATE();
    case 241:
      if (lookahead == 'f') ADVANCE(305);
      END_STATE();
    case 242:
      if (lookahead == 'f') ADVANCE(223);
      END_STATE();
    case 243:
      if (lookahead == 'g') ADVANCE(663);
      END_STATE();
    case 244:
      if (lookahead == 'g') ADVANCE(664);
      END_STATE();
    case 245:
      if (lookahead == 'g') ADVANCE(840);
      END_STATE();
    case 246:
      if (lookahead == 'g') ADVANCE(841);
      END_STATE();
    case 247:
      if (lookahead == 'g') ADVANCE(259);
      END_STATE();
    case 248:
      if (lookahead == 'g') ADVANCE(206);
      END_STATE();
    case 249:
      if (lookahead == 'g') ADVANCE(195);
      END_STATE();
    case 250:
      if (lookahead == 'g') ADVANCE(285);
      if (lookahead == 's') ADVANCE(289);
      END_STATE();
    case 251:
      if (lookahead == 'g') ADVANCE(198);
      END_STATE();
    case 252:
      if (lookahead == 'g') ADVANCE(311);
      if (lookahead == 's') ADVANCE(293);
      END_STATE();
    case 253:
      if (lookahead == 'h') ADVANCE(646);
      END_STATE();
    case 254:
      if (lookahead == 'h') ADVANCE(831);
      END_STATE();
    case 255:
      if (lookahead == 'h') ADVANCE(671);
      END_STATE();
    case 256:
      if (lookahead == 'h') ADVANCE(672);
      END_STATE();
    case 257:
      if (lookahead == 'h') ADVANCE(304);
      END_STATE();
    case 258:
      if (lookahead == 'h') ADVANCE(62);
      END_STATE();
    case 259:
      if (lookahead == 'h') ADVANCE(543);
      END_STATE();
    case 260:
      if (lookahead == 'h') ADVANCE(577);
      END_STATE();
    case 261:
      if (lookahead == 'h') ADVANCE(294);
      END_STATE();
    case 262:
      if (lookahead == 'h') ADVANCE(204);
      END_STATE();
    case 263:
      if (lookahead == 'h') ADVANCE(295);
      END_STATE();
    case 264:
      if (lookahead == 'h') ADVANCE(306);
      END_STATE();
    case 265:
      if (lookahead == 'h') ADVANCE(306);
      if (lookahead == 'i') ADVANCE(554);
      if (lookahead == 'o') ADVANCE(487);
      END_STATE();
    case 266:
      if (lookahead == 'h') ADVANCE(589);
      END_STATE();
    case 267:
      if (lookahead == 'i') ADVANCE(233);
      END_STATE();
    case 268:
      if (lookahead == 'i') ADVANCE(332);
      END_STATE();
    case 269:
      if (lookahead == 'i') ADVANCE(332);
      if (lookahead == 'o') ADVANCE(388);
      if (lookahead == 'u') ADVANCE(336);
      END_STATE();
    case 270:
      if (lookahead == 'i') ADVANCE(250);
      END_STATE();
    case 271:
      if (lookahead == 'i') ADVANCE(247);
      if (lookahead == 'l') ADVANCE(221);
      END_STATE();
    case 272:
      if (lookahead == 'i') ADVANCE(234);
      if (lookahead == 'l') ADVANCE(632);
      if (lookahead == 'o') ADVANCE(598);
      END_STATE();
    case 273:
      if (lookahead == 'i') ADVANCE(234);
      if (lookahead == 'l') ADVANCE(632);
      if (lookahead == 'o') ADVANCE(598);
      if (lookahead == 'r') ADVANCE(411);
      END_STATE();
    case 274:
      if (lookahead == 'i') ADVANCE(235);
      if (lookahead == 'o') ADVANCE(598);
      END_STATE();
    case 275:
      if (lookahead == 'i') ADVANCE(500);
      if (lookahead == 'l') ADVANCE(631);
      END_STATE();
    case 276:
      if (lookahead == 'i') ADVANCE(236);
      END_STATE();
    case 277:
      if (lookahead == 'i') ADVANCE(240);
      END_STATE();
    case 278:
      if (lookahead == 'i') ADVANCE(393);
      END_STATE();
    case 279:
      if (lookahead == 'i') ADVANCE(338);
      END_STATE();
    case 280:
      if (lookahead == 'i') ADVANCE(318);
      END_STATE();
    case 281:
      if (lookahead == 'i') ADVANCE(363);
      END_STATE();
    case 282:
      if (lookahead == 'i') ADVANCE(364);
      END_STATE();
    case 283:
      if (lookahead == 'i') ADVANCE(518);
      if (lookahead == 'r') ADVANCE(201);
      END_STATE();
    case 284:
      if (lookahead == 'i') ADVANCE(379);
      END_STATE();
    case 285:
      if (lookahead == 'i') ADVANCE(337);
      END_STATE();
    case 286:
      if (lookahead == 'i') ADVANCE(86);
      END_STATE();
    case 287:
      if (lookahead == 'i') ADVANCE(382);
      END_STATE();
    case 288:
      if (lookahead == 'i') ADVANCE(98);
      END_STATE();
    case 289:
      if (lookahead == 'i') ADVANCE(541);
      END_STATE();
    case 290:
      if (lookahead == 'i') ADVANCE(380);
      END_STATE();
    case 291:
      if (lookahead == 'i') ADVANCE(436);
      END_STATE();
    case 292:
      if (lookahead == 'i') ADVANCE(383);
      END_STATE();
    case 293:
      if (lookahead == 'i') ADVANCE(544);
      END_STATE();
    case 294:
      if (lookahead == 'i') ADVANCE(384);
      END_STATE();
    case 295:
      if (lookahead == 'i') ADVANCE(389);
      END_STATE();
    case 296:
      if (lookahead == 'i') ADVANCE(172);
      if (lookahead == 'y') ADVANCE(819);
      END_STATE();
    case 297:
      if (lookahead == 'i') ADVANCE(176);
      if (lookahead == 'y') ADVANCE(785);
      END_STATE();
    case 298:
      if (lookahead == 'i') ADVANCE(334);
      END_STATE();
    case 299:
      if (lookahead == 'i') ADVANCE(334);
      if (lookahead == 'o') ADVANCE(410);
      if (lookahead == 'u') ADVANCE(356);
      END_STATE();
    case 300:
      if (lookahead == 'i') ADVANCE(339);
      END_STATE();
    case 301:
      if (lookahead == 'i') ADVANCE(319);
      END_STATE();
    case 302:
      if (lookahead == 'i') ADVANCE(437);
      END_STATE();
    case 303:
      if (lookahead == 'i') ADVANCE(239);
      END_STATE();
    case 304:
      if (lookahead == 'i') ADVANCE(568);
      END_STATE();
    case 305:
      if (lookahead == 'i') ADVANCE(100);
      END_STATE();
    case 306:
      if (lookahead == 'i') ADVANCE(572);
      END_STATE();
    case 307:
      if (lookahead == 'i') ADVANCE(252);
      END_STATE();
    case 308:
      if (lookahead == 'i') ADVANCE(241);
      END_STATE();
    case 309:
      if (lookahead == 'i') ADVANCE(242);
      END_STATE();
    case 310:
      if (lookahead == 'i') ADVANCE(515);
      if (lookahead == 'l') ADVANCE(632);
      END_STATE();
    case 311:
      if (lookahead == 'i') ADVANCE(355);
      END_STATE();
    case 312:
      if (lookahead == 'i') ADVANCE(109);
      END_STATE();
    case 313:
      if (lookahead == 'i') ADVANCE(110);
      END_STATE();
    case 314:
      if (lookahead == 'k') ADVANCE(769);
      END_STATE();
    case 315:
      if (lookahead == 'k') ADVANCE(771);
      END_STATE();
    case 316:
      if (lookahead == 'k') ADVANCE(667);
      END_STATE();
    case 317:
      if (lookahead == 'k') ADVANCE(668);
      END_STATE();
    case 318:
      if (lookahead == 'k') ADVANCE(160);
      END_STATE();
    case 319:
      if (lookahead == 'k') ADVANCE(161);
      END_STATE();
    case 320:
      if (lookahead == 'k') ADVANCE(202);
      END_STATE();
    case 321:
      if (lookahead == 'k') ADVANCE(194);
      END_STATE();
    case 322:
      if (lookahead == 'l') ADVANCE(43);
      END_STATE();
    case 323:
      if (lookahead == 'l') ADVANCE(644);
      END_STATE();
    case 324:
      if (lookahead == 'l') ADVANCE(702);
      END_STATE();
    case 325:
      if (lookahead == 'l') ADVANCE(703);
      END_STATE();
    case 326:
      if (lookahead == 'l') ADVANCE(734);
      END_STATE();
    case 327:
      if (lookahead == 'l') ADVANCE(735);
      END_STATE();
    case 328:
      if (lookahead == 'l') ADVANCE(736);
      END_STATE();
    case 329:
      if (lookahead == 'l') ADVANCE(29);
      if (lookahead == 'r') ADVANCE(451);
      END_STATE();
    case 330:
      if (lookahead == 'l') ADVANCE(424);
      END_STATE();
    case 331:
      if (lookahead == 'l') ADVANCE(424);
      if (lookahead == 'u') ADVANCE(392);
      END_STATE();
    case 332:
      if (lookahead == 'l') ADVANCE(324);
      END_STATE();
    case 333:
      if (lookahead == 'l') ADVANCE(320);
      END_STATE();
    case 334:
      if (lookahead == 'l') ADVANCE(325);
      END_STATE();
    case 335:
      if (lookahead == 'l') ADVANCE(119);
      END_STATE();
    case 336:
      if (lookahead == 'l') ADVANCE(580);
      END_STATE();
    case 337:
      if (lookahead == 'l') ADVANCE(56);
      END_STATE();
    case 338:
      if (lookahead == 'l') ADVANCE(144);
      END_STATE();
    case 339:
      if (lookahead == 'l') ADVANCE(147);
      END_STATE();
    case 340:
      if (lookahead == 'l') ADVANCE(71);
      if (lookahead == 'r') ADVANCE(451);
      END_STATE();
    case 341:
      if (lookahead == 'l') ADVANCE(168);
      END_STATE();
    case 342:
      if (lookahead == 'l') ADVANCE(50);
      END_STATE();
    case 343:
      if (lookahead == 'l') ADVANCE(605);
      if (lookahead == 'n') ADVANCE(560);
      END_STATE();
    case 344:
      if (lookahead == 'l') ADVANCE(434);
      END_STATE();
    case 345:
      if (lookahead == 'l') ADVANCE(428);
      END_STATE();
    case 346:
      if (lookahead == 'l') ADVANCE(208);
      END_STATE();
    case 347:
      if (lookahead == 'l') ADVANCE(290);
      END_STATE();
    case 348:
      if (lookahead == 'l') ADVANCE(292);
      END_STATE();
    case 349:
      if (lookahead == 'l') ADVANCE(447);
      END_STATE();
    case 350:
      if (lookahead == 'l') ADVANCE(448);
      END_STATE();
    case 351:
      if (lookahead == 'l') ADVANCE(449);
      END_STATE();
    case 352:
      if (lookahead == 'l') ADVANCE(75);
      if (lookahead == 'r') ADVANCE(451);
      END_STATE();
    case 353:
      if (lookahead == 'l') ADVANCE(446);
      END_STATE();
    case 354:
      if (lookahead == 'l') ADVANCE(446);
      if (lookahead == 'u') ADVANCE(409);
      END_STATE();
    case 355:
      if (lookahead == 'l') ADVANCE(76);
      END_STATE();
    case 356:
      if (lookahead == 'l') ADVANCE(594);
      END_STATE();
    case 357:
      if (lookahead == 'm') ADVANCE(837);
      END_STATE();
    case 358:
      if (lookahead == 'm') ADVANCE(643);
      END_STATE();
    case 359:
      if (lookahead == 'm') ADVANCE(83);
      END_STATE();
    case 360:
      if (lookahead == 'm') ADVANCE(216);
      END_STATE();
    case 361:
      if (lookahead == 'n') ADVANCE(20);
      if (lookahead == 'r') ADVANCE(117);
      END_STATE();
    case 362:
      if (lookahead == 'n') ADVANCE(853);
      END_STATE();
    case 363:
      if (lookahead == 'n') ADVANCE(706);
      END_STATE();
    case 364:
      if (lookahead == 'n') ADVANCE(707);
      END_STATE();
    case 365:
      if (lookahead == 'n') ADVANCE(777);
      END_STATE();
    case 366:
      if (lookahead == 'n') ADVANCE(779);
      END_STATE();
    case 367:
      if (lookahead == 'n') ADVANCE(850);
      END_STATE();
    case 368:
      if (lookahead == 'n') ADVANCE(854);
      END_STATE();
    case 369:
      if (lookahead == 'n') ADVANCE(858);
      END_STATE();
    case 370:
      if (lookahead == 'n') ADVANCE(859);
      END_STATE();
    case 371:
      if (lookahead == 'n') ADVANCE(861);
      END_STATE();
    case 372:
      if (lookahead == 'n') ADVANCE(857);
      END_STATE();
    case 373:
      if (lookahead == 'n') ADVANCE(862);
      END_STATE();
    case 374:
      if (lookahead == 'n') ADVANCE(856);
      END_STATE();
    case 375:
      if (lookahead == 'n') ADVANCE(860);
      END_STATE();
    case 376:
      if (lookahead == 'n') ADVANCE(835);
      END_STATE();
    case 377:
      if (lookahead == 'n') ADVANCE(836);
      END_STATE();
    case 378:
      if (lookahead == 'n') ADVANCE(88);
      if (lookahead == 'v') ADVANCE(227);
      END_STATE();
    case 379:
      if (lookahead == 'n') ADVANCE(243);
      END_STATE();
    case 380:
      if (lookahead == 'n') ADVANCE(316);
      END_STATE();
    case 381:
      if (lookahead == 'n') ADVANCE(533);
      END_STATE();
    case 382:
      if (lookahead == 'n') ADVANCE(244);
      END_STATE();
    case 383:
      if (lookahead == 'n') ADVANCE(317);
      END_STATE();
    case 384:
      if (lookahead == 'n') ADVANCE(245);
      END_STATE();
    case 385:
      if (lookahead == 'n') ADVANCE(596);
      END_STATE();
    case 386:
      if (lookahead == 'n') ADVANCE(136);
      END_STATE();
    case 387:
      if (lookahead == 'n') ADVANCE(136);
      if (lookahead == 'p') ADVANCE(465);
      END_STATE();
    case 388:
      if (lookahead == 'n') ADVANCE(423);
      END_STATE();
    case 389:
      if (lookahead == 'n') ADVANCE(246);
      END_STATE();
    case 390:
      if (lookahead == 'n') ADVANCE(118);
      END_STATE();
    case 391:
      if (lookahead == 'n') ADVANCE(132);
      END_STATE();
    case 392:
      if (lookahead == 'n') ADVANCE(583);
      END_STATE();
    case 393:
      if (lookahead == 'n') ADVANCE(143);
      END_STATE();
    case 394:
      if (lookahead == 'n') ADVANCE(165);
      END_STATE();
    case 395:
      if (lookahead == 'n') ADVANCE(165);
      if (lookahead == 'y') ADVANCE(180);
      END_STATE();
    case 396:
      if (lookahead == 'n') ADVANCE(551);
      END_STATE();
    case 397:
      if (lookahead == 'n') ADVANCE(547);
      END_STATE();
    case 398:
      if (lookahead == 'n') ADVANCE(550);
      END_STATE();
    case 399:
      if (lookahead == 'n') ADVANCE(213);
      END_STATE();
    case 400:
      if (lookahead == 'n') ADVANCE(549);
      END_STATE();
    case 401:
      if (lookahead == 'n') ADVANCE(558);
      if (lookahead == 'p') ADVANCE(11);
      END_STATE();
    case 402:
      if (lookahead == 'n') ADVANCE(99);
      END_STATE();
    case 403:
      if (lookahead == 'n') ADVANCE(565);
      END_STATE();
    case 404:
      if (lookahead == 'n') ADVANCE(569);
      END_STATE();
    case 405:
      if (lookahead == 'n') ADVANCE(101);
      END_STATE();
    case 406:
      if (lookahead == 'n') ADVANCE(567);
      if (lookahead == 'p') ADVANCE(11);
      END_STATE();
    case 407:
      if (lookahead == 'n') ADVANCE(571);
      END_STATE();
    case 408:
      if (lookahead == 'n') ADVANCE(573);
      END_STATE();
    case 409:
      if (lookahead == 'n') ADVANCE(584);
      END_STATE();
    case 410:
      if (lookahead == 'n') ADVANCE(455);
      END_STATE();
    case 411:
      if (lookahead == 'o') ADVANCE(357);
      END_STATE();
    case 412:
      if (lookahead == 'o') ADVANCE(725);
      END_STATE();
    case 413:
      if (lookahead == 'o') ADVANCE(388);
      if (lookahead == 'u') ADVANCE(336);
      END_STATE();
    case 414:
      if (lookahead == 'o') ADVANCE(595);
      END_STATE();
    case 415:
      if (lookahead == 'o') ADVANCE(600);
      END_STATE();
    case 416:
      if (lookahead == 'o') ADVANCE(610);
      END_STATE();
    case 417:
      if (lookahead == 'o') ADVANCE(358);
      END_STATE();
    case 418:
      if (lookahead == 'o') ADVANCE(488);
      END_STATE();
    case 419:
      if (lookahead == 'o') ADVANCE(229);
      END_STATE();
    case 420:
      if (lookahead == 'o') ADVANCE(230);
      END_STATE();
    case 421:
      if (lookahead == 'o') ADVANCE(231);
      END_STATE();
    case 422:
      if (lookahead == 'o') ADVANCE(232);
      END_STATE();
    case 423:
      if (lookahead == 'o') ADVANCE(94);
      END_STATE();
    case 424:
      if (lookahead == 'o') ADVANCE(507);
      END_STATE();
    case 425:
      if (lookahead == 'o') ADVANCE(626);
      END_STATE();
    case 426:
      if (lookahead == 'o') ADVANCE(627);
      END_STATE();
    case 427:
      if (lookahead == 'o') ADVANCE(344);
      END_STATE();
    case 428:
      if (lookahead == 'o') ADVANCE(92);
      END_STATE();
    case 429:
      if (lookahead == 'o') ADVANCE(419);
      END_STATE();
    case 430:
      if (lookahead == 'o') ADVANCE(326);
      END_STATE();
    case 431:
      if (lookahead == 'o') ADVANCE(420);
      END_STATE();
    case 432:
      if (lookahead == 'o') ADVANCE(327);
      END_STATE();
    case 433:
      if (lookahead == 'o') ADVANCE(328);
      END_STATE();
    case 434:
      if (lookahead == 'o') ADVANCE(510);
      END_STATE();
    case 435:
      if (lookahead == 'o') ADVANCE(422);
      END_STATE();
    case 436:
      if (lookahead == 'o') ADVANCE(376);
      END_STATE();
    case 437:
      if (lookahead == 'o') ADVANCE(377);
      END_STATE();
    case 438:
      if (lookahead == 'o') ADVANCE(399);
      END_STATE();
    case 439:
      if (lookahead == 'o') ADVANCE(564);
      END_STATE();
    case 440:
      if (lookahead == 'o') ADVANCE(330);
      END_STATE();
    case 441:
      if (lookahead == 'o') ADVANCE(561);
      if (lookahead == 'y') ADVANCE(13);
      END_STATE();
    case 442:
      if (lookahead == 'o') ADVANCE(404);
      END_STATE();
    case 443:
      if (lookahead == 'o') ADVANCE(407);
      END_STATE();
    case 444:
      if (lookahead == 'o') ADVANCE(408);
      END_STATE();
    case 445:
      if (lookahead == 'o') ADVANCE(601);
      END_STATE();
    case 446:
      if (lookahead == 'o') ADVANCE(509);
      END_STATE();
    case 447:
      if (lookahead == 'o') ADVANCE(511);
      END_STATE();
    case 448:
      if (lookahead == 'o') ADVANCE(512);
      END_STATE();
    case 449:
      if (lookahead == 'o') ADVANCE(513);
      END_STATE();
    case 450:
      if (lookahead == 'o') ADVANCE(349);
      END_STATE();
    case 451:
      if (lookahead == 'o') ADVANCE(592);
      END_STATE();
    case 452:
      if (lookahead == 'o') ADVANCE(353);
      if (lookahead == 'r') ADVANCE(211);
      END_STATE();
    case 453:
      if (lookahead == 'o') ADVANCE(350);
      END_STATE();
    case 454:
      if (lookahead == 'o') ADVANCE(351);
      END_STATE();
    case 455:
      if (lookahead == 'o') ADVANCE(108);
      END_STATE();
    case 456:
      if (lookahead == 'o') ADVANCE(410);
      if (lookahead == 'u') ADVANCE(356);
      END_STATE();
    case 457:
      if (lookahead == 'p') ADVANCE(683);
      if (lookahead == 'r') ADVANCE(249);
      END_STATE();
    case 458:
      if (lookahead == 'p') ADVANCE(685);
      if (lookahead == 'r') ADVANCE(251);
      END_STATE();
    case 459:
      if (lookahead == 'p') ADVANCE(687);
      END_STATE();
    case 460:
      if (lookahead == 'p') ADVANCE(689);
      END_STATE();
    case 461:
      if (lookahead == 'p') ADVANCE(682);
      if (lookahead == 'r') ADVANCE(249);
      END_STATE();
    case 462:
      if (lookahead == 'p') ADVANCE(684);
      if (lookahead == 'r') ADVANCE(251);
      END_STATE();
    case 463:
      if (lookahead == 'p') ADVANCE(686);
      END_STATE();
    case 464:
      if (lookahead == 'p') ADVANCE(688);
      END_STATE();
    case 465:
      if (lookahead == 'p') ADVANCE(438);
      END_STATE();
    case 466:
      if (lookahead == 'p') ADVANCE(492);
      END_STATE();
    case 467:
      if (lookahead == 'p') ADVANCE(504);
      END_STATE();
    case 468:
      if (lookahead == 'p') ADVANCE(508);
      END_STATE();
    case 469:
      if (lookahead == 'r') ADVANCE(847);
      END_STATE();
    case 470:
      if (lookahead == 'r') ADVANCE(713);
      END_STATE();
    case 471:
      if (lookahead == 'r') ADVANCE(715);
      END_STATE();
    case 472:
      if (lookahead == 'r') ADVANCE(296);
      END_STATE();
    case 473:
      if (lookahead == 'r') ADVANCE(696);
      END_STATE();
    case 474:
      if (lookahead == 'r') ADVANCE(697);
      END_STATE();
    case 475:
      if (lookahead == 'r') ADVANCE(823);
      END_STATE();
    case 476:
      if (lookahead == 'r') ADVANCE(726);
      END_STATE();
    case 477:
      if (lookahead == 'r') ADVANCE(439);
      END_STATE();
    case 478:
      if (lookahead == 'r') ADVANCE(576);
      END_STATE();
    case 479:
      if (lookahead == 'r') ADVANCE(297);
      END_STATE();
    case 480:
      if (lookahead == 'r') ADVANCE(169);
      END_STATE();
    case 481:
      if (lookahead == 'r') ADVANCE(622);
      END_STATE();
    case 482:
      if (lookahead == 'r') ADVANCE(116);
      END_STATE();
    case 483:
      if (lookahead == 'r') ADVANCE(623);
      END_STATE();
    case 484:
      if (lookahead == 'r') ADVANCE(117);
      END_STATE();
    case 485:
      if (lookahead == 'r') ADVANCE(629);
      END_STATE();
    case 486:
      if (lookahead == 'r') ADVANCE(624);
      END_STATE();
    case 487:
      if (lookahead == 'r') ADVANCE(335);
      END_STATE();
    case 488:
      if (lookahead == 'r') ADVANCE(102);
      END_STATE();
    case 489:
      if (lookahead == 'r') ADVANCE(12);
      END_STATE();
    case 490:
      if (lookahead == 'r') ADVANCE(280);
      END_STATE();
    case 491:
      if (lookahead == 'r') ADVANCE(425);
      END_STATE();
    case 492:
      if (lookahead == 'r') ADVANCE(429);
      END_STATE();
    case 493:
      if (lookahead == 'r') ADVANCE(122);
      END_STATE();
    case 494:
      if (lookahead == 'r') ADVANCE(123);
      END_STATE();
    case 495:
      if (lookahead == 'r') ADVANCE(430);
      END_STATE();
    case 496:
      if (lookahead == 'r') ADVANCE(432);
      END_STATE();
    case 497:
      if (lookahead == 'r') ADVANCE(417);
      END_STATE();
    case 498:
      if (lookahead == 'r') ADVANCE(433);
      END_STATE();
    case 499:
      if (lookahead == 'r') ADVANCE(155);
      END_STATE();
    case 500:
      if (lookahead == 'r') ADVANCE(531);
      END_STATE();
    case 501:
      if (lookahead == 'r') ADVANCE(249);
      END_STATE();
    case 502:
      if (lookahead == 'r') ADVANCE(301);
      END_STATE();
    case 503:
      if (lookahead == 'r') ADVANCE(426);
      END_STATE();
    case 504:
      if (lookahead == 'r') ADVANCE(431);
      END_STATE();
    case 505:
      if (lookahead == 'r') ADVANCE(277);
      END_STATE();
    case 506:
      if (lookahead == 'r') ADVANCE(251);
      END_STATE();
    case 507:
      if (lookahead == 'r') ADVANCE(341);
      END_STATE();
    case 508:
      if (lookahead == 'r') ADVANCE(435);
      END_STATE();
    case 509:
      if (lookahead == 'r') ADVANCE(346);
      END_STATE();
    case 510:
      if (lookahead == 'r') ADVANCE(186);
      END_STATE();
    case 511:
      if (lookahead == 'r') ADVANCE(189);
      END_STATE();
    case 512:
      if (lookahead == 'r') ADVANCE(190);
      END_STATE();
    case 513:
      if (lookahead == 'r') ADVANCE(192);
      END_STATE();
    case 514:
      if (lookahead == 'r') ADVANCE(209);
      END_STATE();
    case 515:
      if (lookahead == 'r') ADVANCE(537);
      END_STATE();
    case 516:
      if (lookahead == 'r') ADVANCE(308);
      END_STATE();
    case 517:
      if (lookahead == 'r') ADVANCE(633);
      END_STATE();
    case 518:
      if (lookahead == 'r') ADVANCE(591);
      END_STATE();
    case 519:
      if (lookahead == 's') ADVANCE(650);
      END_STATE();
    case 520:
      if (lookahead == 's') ADVANCE(651);
      END_STATE();
    case 521:
      if (lookahead == 's') ADVANCE(781);
      END_STATE();
    case 522:
      if (lookahead == 's') ADVANCE(783);
      END_STATE();
    case 523:
      if (lookahead == 's') ADVANCE(821);
      END_STATE();
    case 524:
      if (lookahead == 's') ADVANCE(787);
      END_STATE();
    case 525:
      if (lookahead == 's') ADVANCE(615);
      END_STATE();
    case 526:
      if (lookahead == 's') ADVANCE(93);
      END_STATE();
    case 527:
      if (lookahead == 's') ADVANCE(286);
      END_STATE();
    case 528:
      if (lookahead == 's') ADVANCE(521);
      END_STATE();
    case 529:
      if (lookahead == 's') ADVANCE(563);
      END_STATE();
    case 530:
      if (lookahead == 's') ADVANCE(522);
      END_STATE();
    case 531:
      if (lookahead == 's') ADVANCE(552);
      END_STATE();
    case 532:
      if (lookahead == 's') ADVANCE(566);
      END_STATE();
    case 533:
      if (lookahead == 's') ADVANCE(588);
      END_STATE();
    case 534:
      if (lookahead == 's') ADVANCE(582);
      END_STATE();
    case 535:
      if (lookahead == 's') ADVANCE(570);
      END_STATE();
    case 536:
      if (lookahead == 's') ADVANCE(570);
      if (lookahead == 'v') ADVANCE(141);
      END_STATE();
    case 537:
      if (lookahead == 's') ADVANCE(585);
      END_STATE();
    case 538:
      if (lookahead == 's') ADVANCE(18);
      END_STATE();
    case 539:
      if (lookahead == 's') ADVANCE(103);
      END_STATE();
    case 540:
      if (lookahead == 't') ADVANCE(677);
      END_STATE();
    case 541:
      if (lookahead == 't') ADVANCE(675);
      END_STATE();
    case 542:
      if (lookahead == 't') ADVANCE(153);
      END_STATE();
    case 543:
      if (lookahead == 't') ADVANCE(851);
      END_STATE();
    case 544:
      if (lookahead == 't') ADVANCE(676);
      END_STATE();
    case 545:
      if (lookahead == 't') ADVANCE(721);
      END_STATE();
    case 546:
      if (lookahead == 't') ADVANCE(722);
      END_STATE();
    case 547:
      if (lookahead == 't') ADVANCE(815);
      END_STATE();
    case 548:
      if (lookahead == 't') ADVANCE(807);
      END_STATE();
    case 549:
      if (lookahead == 't') ADVANCE(811);
      END_STATE();
    case 550:
      if (lookahead == 't') ADVANCE(19);
      END_STATE();
    case 551:
      if (lookahead == 't') ADVANCE(360);
      END_STATE();
    case 552:
      if (lookahead == 't') ADVANCE(15);
      END_STATE();
    case 553:
      if (lookahead == 't') ADVANCE(1);
      END_STATE();
    case 554:
      if (lookahead == 't') ADVANCE(254);
      END_STATE();
    case 555:
      if (lookahead == 't') ADVANCE(261);
      END_STATE();
    case 556:
      if (lookahead == 't') ADVANCE(291);
      END_STATE();
    case 557:
      if (lookahead == 't') ADVANCE(42);
      END_STATE();
    case 558:
      if (lookahead == 't') ADVANCE(45);
      END_STATE();
    case 559:
      if (lookahead == 't') ADVANCE(412);
      END_STATE();
    case 560:
      if (lookahead == 't') ADVANCE(625);
      END_STATE();
    case 561:
      if (lookahead == 't') ADVANCE(262);
      END_STATE();
    case 562:
      if (lookahead == 't') ADVANCE(491);
      END_STATE();
    case 563:
      if (lookahead == 't') ADVANCE(145);
      END_STATE();
    case 564:
      if (lookahead == 't') ADVANCE(174);
      END_STATE();
    case 565:
      if (lookahead == 't') ADVANCE(48);
      END_STATE();
    case 566:
      if (lookahead == 't') ADVANCE(490);
      END_STATE();
    case 567:
      if (lookahead == 't') ADVANCE(51);
      END_STATE();
    case 568:
      if (lookahead == 't') ADVANCE(146);
      END_STATE();
    case 569:
      if (lookahead == 't') ADVANCE(495);
      END_STATE();
    case 570:
      if (lookahead == 't') ADVANCE(148);
      END_STATE();
    case 571:
      if (lookahead == 't') ADVANCE(496);
      END_STATE();
    case 572:
      if (lookahead == 't') ADVANCE(150);
      END_STATE();
    case 573:
      if (lookahead == 't') ADVANCE(498);
      END_STATE();
    case 574:
      if (lookahead == 't') ADVANCE(151);
      END_STATE();
    case 575:
      if (lookahead == 't') ADVANCE(152);
      END_STATE();
    case 576:
      if (lookahead == 't') ADVANCE(276);
      END_STATE();
    case 577:
      if (lookahead == 't') ADVANCE(415);
      END_STATE();
    case 578:
      if (lookahead == 't') ADVANCE(599);
      END_STATE();
    case 579:
      if (lookahead == 't') ADVANCE(260);
      END_STATE();
    case 580:
      if (lookahead == 't') ADVANCE(312);
      END_STATE();
    case 581:
      if (lookahead == 't') ADVANCE(503);
      END_STATE();
    case 582:
      if (lookahead == 't') ADVANCE(502);
      END_STATE();
    case 583:
      if (lookahead == 't') ADVANCE(185);
      END_STATE();
    case 584:
      if (lookahead == 't') ADVANCE(187);
      END_STATE();
    case 585:
      if (lookahead == 't') ADVANCE(16);
      END_STATE();
    case 586:
      if (lookahead == 't') ADVANCE(263);
      END_STATE();
    case 587:
      if (lookahead == 't') ADVANCE(302);
      END_STATE();
    case 588:
      if (lookahead == 't') ADVANCE(68);
      END_STATE();
    case 589:
      if (lookahead == 't') ADVANCE(445);
      END_STATE();
    case 590:
      if (lookahead == 't') ADVANCE(214);
      END_STATE();
    case 591:
      if (lookahead == 't') ADVANCE(219);
      END_STATE();
    case 592:
      if (lookahead == 't') ADVANCE(226);
      END_STATE();
    case 593:
      if (lookahead == 't') ADVANCE(266);
      END_STATE();
    case 594:
      if (lookahead == 't') ADVANCE(313);
      END_STATE();
    case 595:
      if (lookahead == 'u') ADVANCE(14);
      END_STATE();
    case 596:
      if (lookahead == 'u') ADVANCE(359);
      END_STATE();
    case 597:
      if (lookahead == 'u') ADVANCE(392);
      END_STATE();
    case 598:
      if (lookahead == 'u') ADVANCE(469);
      END_STATE();
    case 599:
      if (lookahead == 'u') ADVANCE(499);
      END_STATE();
    case 600:
      if (lookahead == 'u') ADVANCE(90);
      END_STATE();
    case 601:
      if (lookahead == 'u') ADVANCE(91);
      END_STATE();
    case 602:
      if (lookahead == 'u') ADVANCE(409);
      END_STATE();
    case 603:
      if (lookahead == 'v') ADVANCE(210);
      END_STATE();
    case 604:
      if (lookahead == 'v') ADVANCE(210);
      if (lookahead == 'x') ADVANCE(279);
      END_STATE();
    case 605:
      if (lookahead == 'v') ADVANCE(154);
      END_STATE();
    case 606:
      if (lookahead == 'v') ADVANCE(179);
      END_STATE();
    case 607:
      if (lookahead == 'v') ADVANCE(181);
      END_STATE();
    case 608:
      if (lookahead == 'w') ADVANCE(699);
      END_STATE();
    case 609:
      if (lookahead == 'w') ADVANCE(701);
      END_STATE();
    case 610:
      if (lookahead == 'w') ADVANCE(793);
      END_STATE();
    case 611:
      if (lookahead == 'w') ADVANCE(698);
      END_STATE();
    case 612:
      if (lookahead == 'w') ADVANCE(700);
      END_STATE();
    case 613:
      if (lookahead == 'w') ADVANCE(519);
      END_STATE();
    case 614:
      if (lookahead == 'w') ADVANCE(520);
      END_STATE();
    case 615:
      if (lookahead == 'w') ADVANCE(54);
      END_STATE();
    case 616:
      if (lookahead == 'x') ADVANCE(849);
      END_STATE();
    case 617:
      if (lookahead == 'x') ADVANCE(466);
      END_STATE();
    case 618:
      if (lookahead == 'x') ADVANCE(279);
      END_STATE();
    case 619:
      if (lookahead == 'x') ADVANCE(467);
      END_STATE();
    case 620:
      if (lookahead == 'x') ADVANCE(468);
      END_STATE();
    case 621:
      if (lookahead == 'y') ADVANCE(758);
      END_STATE();
    case 622:
      if (lookahead == 'y') ADVANCE(704);
      END_STATE();
    case 623:
      if (lookahead == 'y') ADVANCE(705);
      END_STATE();
    case 624:
      if (lookahead == 'y') ADVANCE(648);
      END_STATE();
    case 625:
      if (lookahead == 'y') ADVANCE(863);
      END_STATE();
    case 626:
      if (lookahead == 'y') ADVANCE(678);
      END_STATE();
    case 627:
      if (lookahead == 'y') ADVANCE(681);
      END_STATE();
    case 628:
      if (lookahead == 'y') ADVANCE(178);
      END_STATE();
    case 629:
      if (lookahead == 'y') ADVANCE(555);
      END_STATE();
    case 630:
      if (lookahead == 'y') ADVANCE(180);
      END_STATE();
    case 631:
      if (lookahead == 'y') ADVANCE(284);
      END_STATE();
    case 632:
      if (lookahead == 'y') ADVANCE(287);
      END_STATE();
    case 633:
      if (lookahead == 'y') ADVANCE(586);
      END_STATE();
    case 634:
      if (lookahead == '}') ADVANCE(757);
      END_STATE();
    case 635:
      if (lookahead == '}') ADVANCE(710);
      END_STATE();
    case 636:
      if (eof) ADVANCE(638);
      if (lookahead == ' ') SKIP(636)
      if (lookahead == 'A') ADVANCE(111);
      if (lookahead == 'C') ADVANCE(22);
      if (lookahead == 'D') ADVANCE(134);
      if (lookahead == 'E') ADVANCE(618);
      if (lookahead == 'F') ADVANCE(275);
      if (lookahead == 'G') ADVANCE(61);
      if (lookahead == 'H') ADVANCE(30);
      if (lookahead == 'L') ADVANCE(267);
      if (lookahead == 'M') ADVANCE(268);
      if (lookahead == 'P') ADVANCE(32);
      if (lookahead == 'S') ADVANCE(23);
      if (lookahead == 'T') ADVANCE(41);
      if (lookahead == 'U') ADVANCE(403);
      if (lookahead == 'V') ADVANCE(270);
      if (lookahead == 'a') ADVANCE(131);
      if (lookahead == 'c') ADVANCE(70);
      if (lookahead == 'd') ADVANCE(225);
      if (lookahead == 'e') ADVANCE(34);
      if (lookahead == 'f') ADVANCE(272);
      if (lookahead == 'g') ADVANCE(66);
      if (lookahead == 'h') ADVANCE(74);
      if (lookahead == 'l') ADVANCE(309);
      if (lookahead == 'm') ADVANCE(298);
      if (lookahead == 'n') ADVANCE(278);
      if (lookahead == 'o') ADVANCE(386);
      if (lookahead == 'p') ADVANCE(352);
      if (lookahead == 's') ADVANCE(78);
      if (lookahead == 't') ADVANCE(44);
      if (lookahead == 'u') ADVANCE(406);
      if (lookahead == 'v') ADVANCE(307);
      if (lookahead == '{') ADVANCE(755);
      if (lookahead == '~') ADVANCE(711);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(747);
      END_STATE();
    case 637:
      if (eof) ADVANCE(638);
      if (lookahead == ' ') SKIP(637)
      if (lookahead == ',') ADVANCE(639);
      if (lookahead == '.') ADVANCE(640);
      if (lookahead == ':') ADVANCE(662);
      if (lookahead == 'A') ADVANCE(1028);
      if (lookahead == 'B') ADVANCE(954);
      if (lookahead == 'C') ADVANCE(988);
      if (lookahead == 'G') ADVANCE(1004);
      if (lookahead == 'R') ADVANCE(903);
      if (lookahead == 'U') ADVANCE(984);
      if (lookahead == 'W') ADVANCE(935);
      if (lookahead == 'a') ADVANCE(1006);
      if (lookahead == 'b') ADVANCE(868);
      if (lookahead == 'c') ADVANCE(992);
      if (lookahead == 'e') ADVANCE(968);
      if (lookahead == 'g') ADVANCE(1012);
      if (lookahead == 'h') ADVANCE(869);
      if (lookahead == 'i') ADVANCE(971);
      if (lookahead == 'l') ADVANCE(874);
      if (lookahead == 'o') ADVANCE(994);
      if (lookahead == 'p') ADVANCE(955);
      if (lookahead == 'r') ADVANCE(910);
      if (lookahead == 's') ADVANCE(973);
      if (lookahead == 'u') ADVANCE(985);
      if (lookahead == 'w') ADVANCE(939);
      if (lookahead == 'y') ADVANCE(986);
      if (lookahead == '~') ADVANCE(711);
      if (lookahead == '-' ||
          ('D' <= lookahead && lookahead <= 'Z') ||
          ('d' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 638:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 639:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 640:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 641:
      ACCEPT_TOKEN(anon_sym_have);
      END_STATE();
    case 642:
      ACCEPT_TOKEN(anon_sym_have);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 643:
      ACCEPT_TOKEN(anon_sym_hexprooffrom);
      END_STATE();
    case 644:
      ACCEPT_TOKEN(anon_sym_all);
      END_STATE();
    case 645:
      ACCEPT_TOKEN(anon_sym_all);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 646:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 647:
      ACCEPT_TOKEN(anon_sym_each);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 648:
      ACCEPT_TOKEN(anon_sym_every);
      END_STATE();
    case 649:
      ACCEPT_TOKEN(anon_sym_every);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 650:
      ACCEPT_TOKEN(anon_sym_Draws);
      END_STATE();
    case 651:
      ACCEPT_TOKEN(anon_sym_draws);
      END_STATE();
    case 652:
      ACCEPT_TOKEN(anon_sym_A);
      END_STATE();
    case 653:
      ACCEPT_TOKEN(anon_sym_A);
      if (lookahead == 'd') ADVANCE(112);
      END_STATE();
    case 654:
      ACCEPT_TOKEN(anon_sym_a);
      END_STATE();
    case 655:
      ACCEPT_TOKEN(anon_sym_a);
      if (lookahead == 'd') ADVANCE(114);
      if (lookahead == 'l') ADVANCE(323);
      if (lookahead == 'r') ADVANCE(576);
      END_STATE();
    case 656:
      ACCEPT_TOKEN(anon_sym_Card);
      END_STATE();
    case 657:
      ACCEPT_TOKEN(anon_sym_card);
      END_STATE();
    case 658:
      ACCEPT_TOKEN(anon_sym_S);
      END_STATE();
    case 659:
      ACCEPT_TOKEN(anon_sym_S);
      if (lookahead == 'a') ADVANCE(89);
      if (lookahead == 'c') ADVANCE(481);
      END_STATE();
    case 660:
      ACCEPT_TOKEN(anon_sym_s);
      END_STATE();
    case 661:
      ACCEPT_TOKEN(anon_sym_s);
      if (lookahead == 'a') ADVANCE(107);
      if (lookahead == 'c') ADVANCE(483);
      if (lookahead == 'e') ADVANCE(606);
      if (lookahead == 'i') ADVANCE(616);
      if (lookahead == 'n') ADVANCE(416);
      if (lookahead == 'o') ADVANCE(488);
      END_STATE();
    case 662:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 663:
      ACCEPT_TOKEN(anon_sym_Flying);
      END_STATE();
    case 664:
      ACCEPT_TOKEN(anon_sym_flying);
      END_STATE();
    case 665:
      ACCEPT_TOKEN(anon_sym_Firststrike);
      END_STATE();
    case 666:
      ACCEPT_TOKEN(anon_sym_firststrike);
      END_STATE();
    case 667:
      ACCEPT_TOKEN(anon_sym_Lifelink);
      END_STATE();
    case 668:
      ACCEPT_TOKEN(anon_sym_lifelink);
      END_STATE();
    case 669:
      ACCEPT_TOKEN(anon_sym_Vigilance);
      END_STATE();
    case 670:
      ACCEPT_TOKEN(anon_sym_vigilance);
      END_STATE();
    case 671:
      ACCEPT_TOKEN(anon_sym_Deathtouch);
      END_STATE();
    case 672:
      ACCEPT_TOKEN(anon_sym_deathtouch);
      END_STATE();
    case 673:
      ACCEPT_TOKEN(anon_sym_Haste);
      END_STATE();
    case 674:
      ACCEPT_TOKEN(anon_sym_haste);
      END_STATE();
    case 675:
      ACCEPT_TOKEN(anon_sym_Visit);
      END_STATE();
    case 676:
      ACCEPT_TOKEN(anon_sym_visit);
      END_STATE();
    case 677:
      ACCEPT_TOKEN(anon_sym_get);
      END_STATE();
    case 678:
      ACCEPT_TOKEN(anon_sym_Destroy);
      END_STATE();
    case 679:
      ACCEPT_TOKEN(anon_sym_Exile);
      END_STATE();
    case 680:
      ACCEPT_TOKEN(anon_sym_exile);
      END_STATE();
    case 681:
      ACCEPT_TOKEN(anon_sym_destroy);
      END_STATE();
    case 682:
      ACCEPT_TOKEN(anon_sym_Tap);
      END_STATE();
    case 683:
      ACCEPT_TOKEN(anon_sym_Tap);
      if (lookahead == 'p') ADVANCE(171);
      END_STATE();
    case 684:
      ACCEPT_TOKEN(anon_sym_tap);
      END_STATE();
    case 685:
      ACCEPT_TOKEN(anon_sym_tap);
      if (lookahead == 'p') ADVANCE(175);
      END_STATE();
    case 686:
      ACCEPT_TOKEN(anon_sym_Untap);
      END_STATE();
    case 687:
      ACCEPT_TOKEN(anon_sym_Untap);
      if (lookahead == 'p') ADVANCE(182);
      END_STATE();
    case 688:
      ACCEPT_TOKEN(anon_sym_untap);
      END_STATE();
    case 689:
      ACCEPT_TOKEN(anon_sym_untap);
      if (lookahead == 'p') ADVANCE(184);
      END_STATE();
    case 690:
      ACCEPT_TOKEN(anon_sym_Discard);
      END_STATE();
    case 691:
      ACCEPT_TOKEN(anon_sym_discard);
      END_STATE();
    case 692:
      ACCEPT_TOKEN(anon_sym_Sacrifice);
      END_STATE();
    case 693:
      ACCEPT_TOKEN(anon_sym_sacrifice);
      END_STATE();
    case 694:
      ACCEPT_TOKEN(anon_sym_Create);
      END_STATE();
    case 695:
      ACCEPT_TOKEN(anon_sym_create);
      END_STATE();
    case 696:
      ACCEPT_TOKEN(anon_sym_Counter);
      END_STATE();
    case 697:
      ACCEPT_TOKEN(anon_sym_counter);
      END_STATE();
    case 698:
      ACCEPT_TOKEN(anon_sym_Draw);
      END_STATE();
    case 699:
      ACCEPT_TOKEN(anon_sym_Draw);
      if (lookahead == 's') ADVANCE(650);
      END_STATE();
    case 700:
      ACCEPT_TOKEN(anon_sym_draw);
      END_STATE();
    case 701:
      ACCEPT_TOKEN(anon_sym_draw);
      if (lookahead == 's') ADVANCE(651);
      END_STATE();
    case 702:
      ACCEPT_TOKEN(anon_sym_Mill);
      END_STATE();
    case 703:
      ACCEPT_TOKEN(anon_sym_mill);
      END_STATE();
    case 704:
      ACCEPT_TOKEN(anon_sym_Scry);
      END_STATE();
    case 705:
      ACCEPT_TOKEN(anon_sym_scry);
      END_STATE();
    case 706:
      ACCEPT_TOKEN(anon_sym_Gain);
      END_STATE();
    case 707:
      ACCEPT_TOKEN(anon_sym_gain);
      END_STATE();
    case 708:
      ACCEPT_TOKEN(anon_sym_Add);
      END_STATE();
    case 709:
      ACCEPT_TOKEN(anon_sym_add);
      END_STATE();
    case 710:
      ACCEPT_TOKEN(anon_sym_LBRACET_RBRACE);
      END_STATE();
    case 711:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 712:
      ACCEPT_TOKEN(anon_sym_can_SQUOTEtbeblocked);
      END_STATE();
    case 713:
      ACCEPT_TOKEN(anon_sym_Player);
      if (lookahead == 's') ADVANCE(717);
      END_STATE();
    case 714:
      ACCEPT_TOKEN(anon_sym_Player);
      if (lookahead == 's') ADVANCE(718);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 715:
      ACCEPT_TOKEN(anon_sym_player);
      if (lookahead == 's') ADVANCE(719);
      END_STATE();
    case 716:
      ACCEPT_TOKEN(anon_sym_player);
      if (lookahead == 's') ADVANCE(720);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 717:
      ACCEPT_TOKEN(anon_sym_Players);
      END_STATE();
    case 718:
      ACCEPT_TOKEN(anon_sym_Players);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 719:
      ACCEPT_TOKEN(anon_sym_players);
      END_STATE();
    case 720:
      ACCEPT_TOKEN(anon_sym_players);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 721:
      ACCEPT_TOKEN(anon_sym_Target);
      END_STATE();
    case 722:
      ACCEPT_TOKEN(anon_sym_target);
      END_STATE();
    case 723:
      ACCEPT_TOKEN(anon_sym_creature);
      if (lookahead == 's') ADVANCE(801);
      END_STATE();
    case 724:
      ACCEPT_TOKEN(anon_sym_creature);
      if (lookahead == 's') ADVANCE(802);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 725:
      ACCEPT_TOKEN(anon_sym_upto);
      END_STATE();
    case 726:
      ACCEPT_TOKEN(anon_sym_another);
      END_STATE();
    case 727:
      ACCEPT_TOKEN(anon_sym_anynumberof);
      END_STATE();
    case 728:
      ACCEPT_TOKEN(anon_sym_Untapped);
      END_STATE();
    case 729:
      ACCEPT_TOKEN(anon_sym_Untapped);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 730:
      ACCEPT_TOKEN(anon_sym_untapped);
      END_STATE();
    case 731:
      ACCEPT_TOKEN(anon_sym_untapped);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 732:
      ACCEPT_TOKEN(anon_sym_Attacking);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 733:
      ACCEPT_TOKEN(anon_sym_attacking);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 734:
      ACCEPT_TOKEN(anon_sym_youcontrol);
      END_STATE();
    case 735:
      ACCEPT_TOKEN(anon_sym_yourcontrol);
      END_STATE();
    case 736:
      ACCEPT_TOKEN(anon_sym_opponent_SQUOTEscontrol);
      END_STATE();
    case 737:
      ACCEPT_TOKEN(anon_sym_W);
      END_STATE();
    case 738:
      ACCEPT_TOKEN(anon_sym_W);
      if (lookahead == 'h') ADVANCE(304);
      END_STATE();
    case 739:
      ACCEPT_TOKEN(anon_sym_U);
      END_STATE();
    case 740:
      ACCEPT_TOKEN(anon_sym_U);
      if (lookahead == 'n') ADVANCE(557);
      END_STATE();
    case 741:
      ACCEPT_TOKEN(anon_sym_B);
      END_STATE();
    case 742:
      ACCEPT_TOKEN(anon_sym_B);
      if (lookahead == 'l') ADVANCE(43);
      END_STATE();
    case 743:
      ACCEPT_TOKEN(anon_sym_R);
      END_STATE();
    case 744:
      ACCEPT_TOKEN(anon_sym_R);
      if (lookahead == 'e') ADVANCE(113);
      END_STATE();
    case 745:
      ACCEPT_TOKEN(anon_sym_G);
      END_STATE();
    case 746:
      ACCEPT_TOKEN(anon_sym_G);
      if (lookahead == 'a') ADVANCE(281);
      if (lookahead == 'r') ADVANCE(169);
      END_STATE();
    case 747:
      ACCEPT_TOKEN(aux_sym_generic_mana_cost_symbol_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(747);
      END_STATE();
    case 748:
      ACCEPT_TOKEN(anon_sym_X);
      END_STATE();
    case 749:
      ACCEPT_TOKEN(sym_colorless_mana_cost_symbol);
      END_STATE();
    case 750:
      ACCEPT_TOKEN(sym_colorless_mana_cost_symbol);
      if (lookahead == 'a') ADVANCE(482);
      if (lookahead == 'o') ADVANCE(331);
      if (lookahead == 'r') ADVANCE(167);
      END_STATE();
    case 751:
      ACCEPT_TOKEN(anon_sym_2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(747);
      END_STATE();
    case 752:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 753:
      ACCEPT_TOKEN(anon_sym_P);
      END_STATE();
    case 754:
      ACCEPT_TOKEN(anon_sym_P);
      if (lookahead == 'a') ADVANCE(621);
      if (lookahead == 'l') ADVANCE(40);
      if (lookahead == 'r') ADVANCE(439);
      END_STATE();
    case 755:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      if (lookahead == 'Q') ADVANCE(634);
      if (lookahead == 'T') ADVANCE(635);
      END_STATE();
    case 756:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 757:
      ACCEPT_TOKEN(anon_sym_LBRACEQ_RBRACE);
      END_STATE();
    case 758:
      ACCEPT_TOKEN(anon_sym_Pay);
      END_STATE();
    case 759:
      ACCEPT_TOKEN(anon_sym_life);
      END_STATE();
    case 760:
      ACCEPT_TOKEN(anon_sym_life);
      if (lookahead == 'l') ADVANCE(292);
      END_STATE();
    case 761:
      ACCEPT_TOKEN(anon_sym_White);
      END_STATE();
    case 762:
      ACCEPT_TOKEN(anon_sym_White);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 763:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 764:
      ACCEPT_TOKEN(anon_sym_white);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 765:
      ACCEPT_TOKEN(anon_sym_Blue);
      END_STATE();
    case 766:
      ACCEPT_TOKEN(anon_sym_Blue);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 767:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 768:
      ACCEPT_TOKEN(anon_sym_blue);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 769:
      ACCEPT_TOKEN(anon_sym_Black);
      END_STATE();
    case 770:
      ACCEPT_TOKEN(anon_sym_Black);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 771:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 772:
      ACCEPT_TOKEN(anon_sym_black);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 773:
      ACCEPT_TOKEN(anon_sym_Red);
      END_STATE();
    case 774:
      ACCEPT_TOKEN(anon_sym_Red);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 775:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 776:
      ACCEPT_TOKEN(anon_sym_red);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 777:
      ACCEPT_TOKEN(anon_sym_Green);
      END_STATE();
    case 778:
      ACCEPT_TOKEN(anon_sym_Green);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 779:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 780:
      ACCEPT_TOKEN(anon_sym_green);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 781:
      ACCEPT_TOKEN(anon_sym_Colorless);
      END_STATE();
    case 782:
      ACCEPT_TOKEN(anon_sym_Colorless);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 783:
      ACCEPT_TOKEN(anon_sym_colorless);
      END_STATE();
    case 784:
      ACCEPT_TOKEN(anon_sym_colorless);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 785:
      ACCEPT_TOKEN(anon_sym_legendary);
      END_STATE();
    case 786:
      ACCEPT_TOKEN(anon_sym_legendary);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 787:
      ACCEPT_TOKEN(anon_sym_legendaries);
      END_STATE();
    case 788:
      ACCEPT_TOKEN(anon_sym_legendaries);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 789:
      ACCEPT_TOKEN(anon_sym_basic);
      if (lookahead == 's') ADVANCE(791);
      END_STATE();
    case 790:
      ACCEPT_TOKEN(anon_sym_basic);
      if (lookahead == 's') ADVANCE(792);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 791:
      ACCEPT_TOKEN(anon_sym_basics);
      END_STATE();
    case 792:
      ACCEPT_TOKEN(anon_sym_basics);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 793:
      ACCEPT_TOKEN(anon_sym_snow);
      if (lookahead == 's') ADVANCE(795);
      END_STATE();
    case 794:
      ACCEPT_TOKEN(anon_sym_snow);
      if (lookahead == 's') ADVANCE(796);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 795:
      ACCEPT_TOKEN(anon_sym_snows);
      END_STATE();
    case 796:
      ACCEPT_TOKEN(anon_sym_snows);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 797:
      ACCEPT_TOKEN(anon_sym_world);
      if (lookahead == 's') ADVANCE(799);
      END_STATE();
    case 798:
      ACCEPT_TOKEN(anon_sym_world);
      if (lookahead == 's') ADVANCE(800);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 799:
      ACCEPT_TOKEN(anon_sym_worlds);
      END_STATE();
    case 800:
      ACCEPT_TOKEN(anon_sym_worlds);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 801:
      ACCEPT_TOKEN(anon_sym_creatures);
      END_STATE();
    case 802:
      ACCEPT_TOKEN(anon_sym_creatures);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 803:
      ACCEPT_TOKEN(anon_sym_land);
      if (lookahead == 's') ADVANCE(805);
      END_STATE();
    case 804:
      ACCEPT_TOKEN(anon_sym_land);
      if (lookahead == 's') ADVANCE(806);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 805:
      ACCEPT_TOKEN(anon_sym_lands);
      END_STATE();
    case 806:
      ACCEPT_TOKEN(anon_sym_lands);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 807:
      ACCEPT_TOKEN(anon_sym_artifact);
      if (lookahead == 's') ADVANCE(809);
      END_STATE();
    case 808:
      ACCEPT_TOKEN(anon_sym_artifact);
      if (lookahead == 's') ADVANCE(810);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 809:
      ACCEPT_TOKEN(anon_sym_artifacts);
      END_STATE();
    case 810:
      ACCEPT_TOKEN(anon_sym_artifacts);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 811:
      ACCEPT_TOKEN(anon_sym_enchantment);
      if (lookahead == 's') ADVANCE(813);
      END_STATE();
    case 812:
      ACCEPT_TOKEN(anon_sym_enchantment);
      if (lookahead == 's') ADVANCE(814);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 813:
      ACCEPT_TOKEN(anon_sym_enchantments);
      END_STATE();
    case 814:
      ACCEPT_TOKEN(anon_sym_enchantments);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 815:
      ACCEPT_TOKEN(anon_sym_instant);
      if (lookahead == 's') ADVANCE(817);
      END_STATE();
    case 816:
      ACCEPT_TOKEN(anon_sym_instant);
      if (lookahead == 's') ADVANCE(818);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 817:
      ACCEPT_TOKEN(anon_sym_instants);
      END_STATE();
    case 818:
      ACCEPT_TOKEN(anon_sym_instants);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 819:
      ACCEPT_TOKEN(anon_sym_sorcery);
      END_STATE();
    case 820:
      ACCEPT_TOKEN(anon_sym_sorcery);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 821:
      ACCEPT_TOKEN(anon_sym_sorceries);
      END_STATE();
    case 822:
      ACCEPT_TOKEN(anon_sym_sorceries);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 823:
      ACCEPT_TOKEN(anon_sym_planeswalker);
      if (lookahead == 's') ADVANCE(825);
      END_STATE();
    case 824:
      ACCEPT_TOKEN(anon_sym_planeswalker);
      if (lookahead == 's') ADVANCE(826);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 825:
      ACCEPT_TOKEN(anon_sym_planeswalkers);
      END_STATE();
    case 826:
      ACCEPT_TOKEN(anon_sym_planeswalkers);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 827:
      ACCEPT_TOKEN(anon_sym_Hexproof);
      END_STATE();
    case 828:
      ACCEPT_TOKEN(anon_sym_hexproof);
      if (lookahead == ' ') ADVANCE(238);
      END_STATE();
    case 829:
      ACCEPT_TOKEN(anon_sym_Tapped);
      END_STATE();
    case 830:
      ACCEPT_TOKEN(anon_sym_tapped);
      END_STATE();
    case 831:
      ACCEPT_TOKEN(anon_sym_with);
      END_STATE();
    case 832:
      ACCEPT_TOKEN(anon_sym_with);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 833:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 834:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 835:
      ACCEPT_TOKEN(anon_sym_Protection);
      END_STATE();
    case 836:
      ACCEPT_TOKEN(anon_sym_protection);
      END_STATE();
    case 837:
      ACCEPT_TOKEN(anon_sym_from);
      END_STATE();
    case 838:
      ACCEPT_TOKEN(anon_sym_Multicolored);
      END_STATE();
    case 839:
      ACCEPT_TOKEN(anon_sym_multicolored);
      END_STATE();
    case 840:
      ACCEPT_TOKEN(anon_sym_Everything);
      END_STATE();
    case 841:
      ACCEPT_TOKEN(anon_sym_everything);
      END_STATE();
    case 842:
      ACCEPT_TOKEN(anon_sym_Monocolored);
      END_STATE();
    case 843:
      ACCEPT_TOKEN(anon_sym_monocolored);
      END_STATE();
    case 844:
      ACCEPT_TOKEN(anon_sym_one);
      END_STATE();
    case 845:
      ACCEPT_TOKEN(anon_sym_two);
      END_STATE();
    case 846:
      ACCEPT_TOKEN(anon_sym_three);
      END_STATE();
    case 847:
      ACCEPT_TOKEN(anon_sym_four);
      if (lookahead == 't') ADVANCE(217);
      END_STATE();
    case 848:
      ACCEPT_TOKEN(anon_sym_five);
      END_STATE();
    case 849:
      ACCEPT_TOKEN(anon_sym_six);
      if (lookahead == 't') ADVANCE(215);
      END_STATE();
    case 850:
      ACCEPT_TOKEN(anon_sym_seven);
      if (lookahead == 't') ADVANCE(220);
      END_STATE();
    case 851:
      ACCEPT_TOKEN(anon_sym_eight);
      if (lookahead == 'e') ADVANCE(193);
      END_STATE();
    case 852:
      ACCEPT_TOKEN(anon_sym_nine);
      if (lookahead == 't') ADVANCE(218);
      END_STATE();
    case 853:
      ACCEPT_TOKEN(anon_sym_ten);
      END_STATE();
    case 854:
      ACCEPT_TOKEN(anon_sym_eleven);
      END_STATE();
    case 855:
      ACCEPT_TOKEN(anon_sym_twelve);
      END_STATE();
    case 856:
      ACCEPT_TOKEN(anon_sym_thirteen);
      END_STATE();
    case 857:
      ACCEPT_TOKEN(anon_sym_fourteen);
      END_STATE();
    case 858:
      ACCEPT_TOKEN(anon_sym_fifteen);
      END_STATE();
    case 859:
      ACCEPT_TOKEN(anon_sym_sixteen);
      END_STATE();
    case 860:
      ACCEPT_TOKEN(anon_sym_seventeen);
      END_STATE();
    case 861:
      ACCEPT_TOKEN(anon_sym_eighteen);
      END_STATE();
    case 862:
      ACCEPT_TOKEN(anon_sym_nineteen);
      END_STATE();
    case 863:
      ACCEPT_TOKEN(anon_sym_twenty);
      END_STATE();
    case 864:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == ' ') ADVANCE(385);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 865:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == ' ') ADVANCE(95);
      if (lookahead == 'r') ADVANCE(866);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 866:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == ' ') ADVANCE(104);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 867:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == '\'') ADVANCE(538);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 868:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(1019);
      if (lookahead == 'l') ADVANCE(873);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 869:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(1041);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 870:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(979);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 871:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(887);
      if (lookahead == 'u') ADVANCE(904);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 872:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(1046);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 873:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(890);
      if (lookahead == 'u') ADVANCE(905);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 874:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(974);
      if (lookahead == 'e') ADVANCE(934);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 875:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(1005);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 876:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(980);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 877:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(891);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 878:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(1031);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 879:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(959);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 880:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(893);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 881:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(977);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 882:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(995);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 883:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(892);
      if (lookahead == 'n') ADVANCE(889);
      if (lookahead == 'v') ADVANCE(919);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 884:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(978);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 885:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(895);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 886:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'a') ADVANCE(999);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('b' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 887:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(949);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 888:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(790);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 889:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(938);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 890:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(950);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 891:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(951);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 892:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(937);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 893:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(1025);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 894:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(912);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 895:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'c') ADVANCE(953);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 896:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'd') ADVANCE(774);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 897:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'd') ADVANCE(776);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 898:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'd') ADVANCE(804);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 899:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'd') ADVANCE(798);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 900:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'd') ADVANCE(729);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 901:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'd') ADVANCE(731);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 902:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'd') ADVANCE(875);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 903:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(896);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 904:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(766);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 905:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(768);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 906:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(762);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 907:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(764);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 908:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(724);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 909:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(642);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 910:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(897);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 911:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1018);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 912:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1000);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 913:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1020);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 914:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(878);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 915:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(917);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 916:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1001);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 917:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(969);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 918:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(900);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 919:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1007);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 920:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(970);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 921:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(901);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 922:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1016);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 923:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1002);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 924:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(976);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 925:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1017);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 926:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1003);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 927:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(1021);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 928:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(920);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 929:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(981);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 930:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'e') ADVANCE(982);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 931:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'f') ADVANCE(880);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 932:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'g') ADVANCE(732);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 933:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'g') ADVANCE(733);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 934:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'g') ADVANCE(924);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 935:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'h') ADVANCE(942);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 936:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'h') ADVANCE(832);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 937:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'h') ADVANCE(647);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 938:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'h') ADVANCE(881);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 939:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'h') ADVANCE(948);
      if (lookahead == 'i') ADVANCE(1032);
      if (lookahead == 'o') ADVANCE(1008);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 940:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'h') ADVANCE(948);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 941:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(931);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 942:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(1034);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 943:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(888);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 944:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(972);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 945:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(975);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 946:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(922);
      if (lookahead == 'y') ADVANCE(820);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 947:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(925);
      if (lookahead == 'y') ADVANCE(786);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 948:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'i') ADVANCE(1035);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 949:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'k') ADVANCE(770);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 950:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'k') ADVANCE(772);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 951:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'k') ADVANCE(944);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 952:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'k') ADVANCE(916);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 953:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'k') ADVANCE(945);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 954:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(871);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 955:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(876);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 956:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(645);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 957:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(989);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 958:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(899);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 959:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(952);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 960:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(956);
      if (lookahead == 'n') ADVANCE(1045);
      if (lookahead == 'r') ADVANCE(1029);
      if (lookahead == 't') ADVANCE(1037);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 961:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(872);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 962:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(870);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 963:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(913);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 964:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(873);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 965:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(927);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 966:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'l') ADVANCE(991);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 967:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'm') ADVANCE(930);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 968:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(889);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 969:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(778);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 970:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(780);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 971:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(1022);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 972:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(932);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 973:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(987);
      if (lookahead == 'o') ADVANCE(1009);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 974:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(898);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 975:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(933);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 976:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(902);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 977:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(1023);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 978:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(1024);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 979:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(911);
      if (lookahead == 'y') ADVANCE(926);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 980:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(911);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 981:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(1026);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 982:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(1027);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 983:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(929);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 984:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(1033);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 985:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'n') ADVANCE(1038);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 986:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(1039);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 987:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(1042);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 988:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(957);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 989:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(1010);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 990:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(983);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 991:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(1013);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 992:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(966);
      if (lookahead == 'r') ADVANCE(914);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 993:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'o') ADVANCE(966);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 994:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'p') ADVANCE(996);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 995:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'p') ADVANCE(997);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 996:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'p') ADVANCE(990);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 997:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'p') ADVANCE(918);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 998:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'p') ADVANCE(921);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 999:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'p') ADVANCE(998);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1000:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(946);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1001:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(824);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1002:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(714);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1003:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(716);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1004:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(915);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1005:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(947);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1006:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(1029);
      if (lookahead == 't') ADVANCE(1037);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1007:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(1044);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1008:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(958);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1009:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(894);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1010:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(963);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1011:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(908);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1012:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(928);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1013:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'r') ADVANCE(965);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1014:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(782);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1015:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(784);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1016:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(822);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1017:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(788);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1018:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(1043);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1019:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(943);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1020:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(1014);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1021:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(1015);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1022:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 's') ADVANCE(1036);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1023:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(967);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1024:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(816);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1025:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(808);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1026:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(867);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1027:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(812);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1028:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(1030);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1029:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(941);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1030:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(877);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1031:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(1040);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1032:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(936);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1033:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(882);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1034:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(906);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1035:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(907);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1036:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(884);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1037:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(885);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1038:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 't') ADVANCE(886);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1039:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'u') ADVANCE(865);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1040:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'u') ADVANCE(1011);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1041:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'v') ADVANCE(909);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1042:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'w') ADVANCE(794);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1043:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'w') ADVANCE(879);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1044:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'y') ADVANCE(649);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1045:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'y') ADVANCE(864);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1046:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == 'y') ADVANCE(923);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    case 1047:
      ACCEPT_TOKEN(sym_any_subtype);
      if (lookahead == '-' ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(1047);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 636},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2},
  [4] = {.lex_state = 3},
  [5] = {.lex_state = 3},
  [6] = {.lex_state = 637},
  [7] = {.lex_state = 637},
  [8] = {.lex_state = 4},
  [9] = {.lex_state = 3},
  [10] = {.lex_state = 3},
  [11] = {.lex_state = 637},
  [12] = {.lex_state = 3},
  [13] = {.lex_state = 3},
  [14] = {.lex_state = 637},
  [15] = {.lex_state = 3},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 637},
  [18] = {.lex_state = 3},
  [19] = {.lex_state = 3},
  [20] = {.lex_state = 3},
  [21] = {.lex_state = 3},
  [22] = {.lex_state = 3},
  [23] = {.lex_state = 637},
  [24] = {.lex_state = 637},
  [25] = {.lex_state = 637},
  [26] = {.lex_state = 637},
  [27] = {.lex_state = 637},
  [28] = {.lex_state = 637},
  [29] = {.lex_state = 3},
  [30] = {.lex_state = 636},
  [31] = {.lex_state = 636},
  [32] = {.lex_state = 636},
  [33] = {.lex_state = 2},
  [34] = {.lex_state = 636},
  [35] = {.lex_state = 636},
  [36] = {.lex_state = 5},
  [37] = {.lex_state = 636},
  [38] = {.lex_state = 636},
  [39] = {.lex_state = 636},
  [40] = {.lex_state = 6},
  [41] = {.lex_state = 636},
  [42] = {.lex_state = 7},
  [43] = {.lex_state = 7},
  [44] = {.lex_state = 636},
  [45] = {.lex_state = 636},
  [46] = {.lex_state = 5},
  [47] = {.lex_state = 5},
  [48] = {.lex_state = 5},
  [49] = {.lex_state = 0},
  [50] = {.lex_state = 0},
  [51] = {.lex_state = 636},
  [52] = {.lex_state = 0},
  [53] = {.lex_state = 636},
  [54] = {.lex_state = 0},
  [55] = {.lex_state = 6},
  [56] = {.lex_state = 0},
  [57] = {.lex_state = 6},
  [58] = {.lex_state = 0},
  [59] = {.lex_state = 0},
  [60] = {.lex_state = 0},
  [61] = {.lex_state = 0},
  [62] = {.lex_state = 0},
  [63] = {.lex_state = 0},
  [64] = {.lex_state = 0},
  [65] = {.lex_state = 0},
  [66] = {.lex_state = 0},
  [67] = {.lex_state = 0},
  [68] = {.lex_state = 0},
  [69] = {.lex_state = 0},
  [70] = {.lex_state = 0},
  [71] = {.lex_state = 0},
  [72] = {.lex_state = 636},
  [73] = {.lex_state = 636},
  [74] = {.lex_state = 0},
  [75] = {.lex_state = 0},
  [76] = {.lex_state = 0},
  [77] = {.lex_state = 0},
  [78] = {.lex_state = 0},
  [79] = {.lex_state = 0},
  [80] = {.lex_state = 0},
  [81] = {.lex_state = 8},
  [82] = {.lex_state = 0},
  [83] = {.lex_state = 636},
  [84] = {.lex_state = 0},
  [85] = {.lex_state = 0},
  [86] = {.lex_state = 636},
  [87] = {.lex_state = 0},
  [88] = {.lex_state = 0},
  [89] = {.lex_state = 2},
  [90] = {.lex_state = 636},
  [91] = {.lex_state = 636},
  [92] = {.lex_state = 0},
  [93] = {.lex_state = 0},
  [94] = {.lex_state = 0},
  [95] = {.lex_state = 636},
  [96] = {.lex_state = 2},
  [97] = {.lex_state = 636},
  [98] = {.lex_state = 636},
  [99] = {.lex_state = 0},
  [100] = {.lex_state = 8},
  [101] = {.lex_state = 0},
  [102] = {.lex_state = 0},
  [103] = {.lex_state = 5},
  [104] = {.lex_state = 0},
  [105] = {.lex_state = 0},
  [106] = {.lex_state = 0},
  [107] = {.lex_state = 0},
  [108] = {.lex_state = 0},
  [109] = {.lex_state = 0},
  [110] = {.lex_state = 0},
  [111] = {.lex_state = 0},
  [112] = {.lex_state = 0},
  [113] = {.lex_state = 0},
  [114] = {.lex_state = 0},
  [115] = {.lex_state = 636},
  [116] = {.lex_state = 8},
  [117] = {.lex_state = 0},
  [118] = {.lex_state = 0},
  [119] = {.lex_state = 0},
  [120] = {.lex_state = 0},
  [121] = {.lex_state = 0},
  [122] = {.lex_state = 0},
  [123] = {.lex_state = 0},
  [124] = {.lex_state = 0},
  [125] = {.lex_state = 0},
  [126] = {.lex_state = 0},
  [127] = {.lex_state = 0},
  [128] = {.lex_state = 0},
  [129] = {.lex_state = 0},
  [130] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_COMMA] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_have] = ACTIONS(1),
    [anon_sym_hexprooffrom] = ACTIONS(1),
    [anon_sym_all] = ACTIONS(1),
    [anon_sym_each] = ACTIONS(1),
    [anon_sym_every] = ACTIONS(1),
    [anon_sym_Draws] = ACTIONS(1),
    [anon_sym_draws] = ACTIONS(1),
    [anon_sym_A] = ACTIONS(1),
    [anon_sym_a] = ACTIONS(1),
    [anon_sym_Card] = ACTIONS(1),
    [anon_sym_card] = ACTIONS(1),
    [anon_sym_S] = ACTIONS(1),
    [anon_sym_s] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_Flying] = ACTIONS(1),
    [anon_sym_flying] = ACTIONS(1),
    [anon_sym_Firststrike] = ACTIONS(1),
    [anon_sym_firststrike] = ACTIONS(1),
    [anon_sym_Lifelink] = ACTIONS(1),
    [anon_sym_lifelink] = ACTIONS(1),
    [anon_sym_Vigilance] = ACTIONS(1),
    [anon_sym_vigilance] = ACTIONS(1),
    [anon_sym_Deathtouch] = ACTIONS(1),
    [anon_sym_deathtouch] = ACTIONS(1),
    [anon_sym_Haste] = ACTIONS(1),
    [anon_sym_haste] = ACTIONS(1),
    [anon_sym_Visit] = ACTIONS(1),
    [anon_sym_visit] = ACTIONS(1),
    [anon_sym_get] = ACTIONS(1),
    [anon_sym_Destroy] = ACTIONS(1),
    [anon_sym_Exile] = ACTIONS(1),
    [anon_sym_exile] = ACTIONS(1),
    [anon_sym_destroy] = ACTIONS(1),
    [anon_sym_Tap] = ACTIONS(1),
    [anon_sym_tap] = ACTIONS(1),
    [anon_sym_Untap] = ACTIONS(1),
    [anon_sym_untap] = ACTIONS(1),
    [anon_sym_Discard] = ACTIONS(1),
    [anon_sym_discard] = ACTIONS(1),
    [anon_sym_Sacrifice] = ACTIONS(1),
    [anon_sym_sacrifice] = ACTIONS(1),
    [anon_sym_Create] = ACTIONS(1),
    [anon_sym_create] = ACTIONS(1),
    [anon_sym_Counter] = ACTIONS(1),
    [anon_sym_counter] = ACTIONS(1),
    [anon_sym_Draw] = ACTIONS(1),
    [anon_sym_draw] = ACTIONS(1),
    [anon_sym_Mill] = ACTIONS(1),
    [anon_sym_mill] = ACTIONS(1),
    [anon_sym_Scry] = ACTIONS(1),
    [anon_sym_scry] = ACTIONS(1),
    [anon_sym_Gain] = ACTIONS(1),
    [anon_sym_gain] = ACTIONS(1),
    [anon_sym_Add] = ACTIONS(1),
    [anon_sym_add] = ACTIONS(1),
    [anon_sym_LBRACET_RBRACE] = ACTIONS(1),
    [anon_sym_TILDE] = ACTIONS(1),
    [anon_sym_can_SQUOTEtbeblocked] = ACTIONS(1),
    [anon_sym_Player] = ACTIONS(1),
    [anon_sym_player] = ACTIONS(1),
    [anon_sym_Players] = ACTIONS(1),
    [anon_sym_players] = ACTIONS(1),
    [anon_sym_Target] = ACTIONS(1),
    [anon_sym_target] = ACTIONS(1),
    [anon_sym_creature] = ACTIONS(1),
    [anon_sym_upto] = ACTIONS(1),
    [anon_sym_Untapped] = ACTIONS(1),
    [anon_sym_untapped] = ACTIONS(1),
    [anon_sym_youcontrol] = ACTIONS(1),
    [anon_sym_yourcontrol] = ACTIONS(1),
    [anon_sym_opponent_SQUOTEscontrol] = ACTIONS(1),
    [anon_sym_W] = ACTIONS(1),
    [anon_sym_U] = ACTIONS(1),
    [anon_sym_B] = ACTIONS(1),
    [anon_sym_R] = ACTIONS(1),
    [anon_sym_G] = ACTIONS(1),
    [aux_sym_generic_mana_cost_symbol_token1] = ACTIONS(1),
    [anon_sym_X] = ACTIONS(1),
    [sym_colorless_mana_cost_symbol] = ACTIONS(1),
    [anon_sym_2] = ACTIONS(1),
    [anon_sym_SLASH] = ACTIONS(1),
    [anon_sym_P] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_LBRACEQ_RBRACE] = ACTIONS(1),
    [anon_sym_Pay] = ACTIONS(1),
    [anon_sym_life] = ACTIONS(1),
    [anon_sym_White] = ACTIONS(1),
    [anon_sym_white] = ACTIONS(1),
    [anon_sym_Blue] = ACTIONS(1),
    [anon_sym_blue] = ACTIONS(1),
    [anon_sym_Black] = ACTIONS(1),
    [anon_sym_black] = ACTIONS(1),
    [anon_sym_Red] = ACTIONS(1),
    [anon_sym_red] = ACTIONS(1),
    [anon_sym_Green] = ACTIONS(1),
    [anon_sym_green] = ACTIONS(1),
    [anon_sym_Colorless] = ACTIONS(1),
    [anon_sym_colorless] = ACTIONS(1),
    [anon_sym_legendary] = ACTIONS(1),
    [anon_sym_legendaries] = ACTIONS(1),
    [anon_sym_basic] = ACTIONS(1),
    [anon_sym_basics] = ACTIONS(1),
    [anon_sym_snow] = ACTIONS(1),
    [anon_sym_snows] = ACTIONS(1),
    [anon_sym_world] = ACTIONS(1),
    [anon_sym_worlds] = ACTIONS(1),
    [anon_sym_creatures] = ACTIONS(1),
    [anon_sym_land] = ACTIONS(1),
    [anon_sym_lands] = ACTIONS(1),
    [anon_sym_artifact] = ACTIONS(1),
    [anon_sym_artifacts] = ACTIONS(1),
    [anon_sym_enchantment] = ACTIONS(1),
    [anon_sym_enchantments] = ACTIONS(1),
    [anon_sym_instant] = ACTIONS(1),
    [anon_sym_instants] = ACTIONS(1),
    [anon_sym_sorcery] = ACTIONS(1),
    [anon_sym_sorceries] = ACTIONS(1),
    [anon_sym_planeswalker] = ACTIONS(1),
    [anon_sym_planeswalkers] = ACTIONS(1),
    [anon_sym_Hexproof] = ACTIONS(1),
    [anon_sym_hexproof] = ACTIONS(1),
    [anon_sym_Tapped] = ACTIONS(1),
    [anon_sym_tapped] = ACTIONS(1),
    [anon_sym_with] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_Protection] = ACTIONS(1),
    [anon_sym_protection] = ACTIONS(1),
    [anon_sym_from] = ACTIONS(1),
    [anon_sym_Multicolored] = ACTIONS(1),
    [anon_sym_multicolored] = ACTIONS(1),
    [anon_sym_Everything] = ACTIONS(1),
    [anon_sym_Monocolored] = ACTIONS(1),
    [anon_sym_monocolored] = ACTIONS(1),
    [anon_sym_one] = ACTIONS(1),
    [anon_sym_two] = ACTIONS(1),
    [anon_sym_three] = ACTIONS(1),
    [anon_sym_four] = ACTIONS(1),
    [anon_sym_five] = ACTIONS(1),
    [anon_sym_six] = ACTIONS(1),
    [anon_sym_seven] = ACTIONS(1),
    [anon_sym_eight] = ACTIONS(1),
    [anon_sym_nine] = ACTIONS(1),
    [anon_sym_ten] = ACTIONS(1),
    [anon_sym_eleven] = ACTIONS(1),
    [anon_sym_twelve] = ACTIONS(1),
    [anon_sym_thirteen] = ACTIONS(1),
    [anon_sym_fourteen] = ACTIONS(1),
    [anon_sym_fifteen] = ACTIONS(1),
    [anon_sym_sixteen] = ACTIONS(1),
    [anon_sym_seventeen] = ACTIONS(1),
    [anon_sym_eighteen] = ACTIONS(1),
    [anon_sym_nineteen] = ACTIONS(1),
    [anon_sym_twenty] = ACTIONS(1),
  },
  [1] = {
    [sym_rules_line] = STATE(123),
    [sym_keyword_ability_list] = STATE(122),
    [sym_line_effect] = STATE(122),
    [sym_line_static_ability] = STATE(122),
    [sym_static_ability_subject] = STATE(92),
    [sym_continuous_tense_qualifier] = STATE(44),
    [sym_draw_effect] = STATE(120),
    [sym_effect] = STATE(119),
    [sym_line_activated_ability] = STATE(122),
    [sym_keyword_ability] = STATE(64),
    [sym_one_shot_effect] = STATE(61),
    [sym_action_verb] = STATE(41),
    [sym_mana_ability] = STATE(122),
    [sym_unblockable_ability] = STATE(122),
    [sym_subject_nontarget] = STATE(93),
    [sym_non_finite_quantity] = STATE(10),
    [sym_mana_cost] = STATE(69),
    [sym_action_cost] = STATE(69),
    [sym_cost] = STATE(117),
    [sym_cost_life_payment] = STATE(69),
    [sym_keyword_ability_protection] = STATE(78),
    [aux_sym_line_effect_repeat1] = STATE(30),
    [aux_sym_mana_cost_repeat1] = STATE(49),
    [anon_sym_all] = ACTIONS(3),
    [anon_sym_each] = ACTIONS(3),
    [anon_sym_every] = ACTIONS(3),
    [anon_sym_Flying] = ACTIONS(5),
    [anon_sym_flying] = ACTIONS(5),
    [anon_sym_Firststrike] = ACTIONS(5),
    [anon_sym_firststrike] = ACTIONS(5),
    [anon_sym_Lifelink] = ACTIONS(5),
    [anon_sym_lifelink] = ACTIONS(5),
    [anon_sym_Vigilance] = ACTIONS(5),
    [anon_sym_vigilance] = ACTIONS(5),
    [anon_sym_Deathtouch] = ACTIONS(5),
    [anon_sym_deathtouch] = ACTIONS(5),
    [anon_sym_Haste] = ACTIONS(5),
    [anon_sym_haste] = ACTIONS(5),
    [anon_sym_Visit] = ACTIONS(5),
    [anon_sym_visit] = ACTIONS(5),
    [anon_sym_Destroy] = ACTIONS(7),
    [anon_sym_Exile] = ACTIONS(7),
    [anon_sym_exile] = ACTIONS(7),
    [anon_sym_destroy] = ACTIONS(7),
    [anon_sym_Tap] = ACTIONS(7),
    [anon_sym_tap] = ACTIONS(7),
    [anon_sym_Untap] = ACTIONS(7),
    [anon_sym_untap] = ACTIONS(7),
    [anon_sym_Discard] = ACTIONS(7),
    [anon_sym_discard] = ACTIONS(7),
    [anon_sym_Sacrifice] = ACTIONS(7),
    [anon_sym_sacrifice] = ACTIONS(7),
    [anon_sym_Create] = ACTIONS(7),
    [anon_sym_create] = ACTIONS(7),
    [anon_sym_Counter] = ACTIONS(7),
    [anon_sym_counter] = ACTIONS(7),
    [anon_sym_Draw] = ACTIONS(7),
    [anon_sym_draw] = ACTIONS(7),
    [anon_sym_Mill] = ACTIONS(7),
    [anon_sym_mill] = ACTIONS(7),
    [anon_sym_Scry] = ACTIONS(7),
    [anon_sym_scry] = ACTIONS(7),
    [anon_sym_Gain] = ACTIONS(7),
    [anon_sym_gain] = ACTIONS(7),
    [anon_sym_Add] = ACTIONS(7),
    [anon_sym_add] = ACTIONS(7),
    [anon_sym_LBRACET_RBRACE] = ACTIONS(9),
    [anon_sym_TILDE] = ACTIONS(11),
    [anon_sym_anynumberof] = ACTIONS(13),
    [anon_sym_LBRACE] = ACTIONS(15),
    [anon_sym_LBRACEQ_RBRACE] = ACTIONS(17),
    [anon_sym_Pay] = ACTIONS(19),
    [anon_sym_Protection] = ACTIONS(21),
    [anon_sym_protection] = ACTIONS(21),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 13,
    ACTIONS(23), 1,
      anon_sym_have,
    ACTIONS(25), 1,
      anon_sym_hexprooffrom,
    STATE(64), 1,
      sym_keyword_ability,
    STATE(78), 1,
      sym_keyword_ability_protection,
    STATE(130), 1,
      sym_static_ability_effect,
    ACTIONS(21), 2,
      anon_sym_Protection,
      anon_sym_protection,
    STATE(74), 2,
      sym_qualifier_color,
      sym_qualifier_type,
    STATE(109), 2,
      sym_keyword_ability_list,
      sym_protection_qualifier,
    ACTIONS(27), 6,
      anon_sym_creature,
      anon_sym_land,
      anon_sym_artifact,
      anon_sym_enchantment,
      anon_sym_instant,
      anon_sym_planeswalker,
    ACTIONS(33), 6,
      anon_sym_Multicolored,
      anon_sym_multicolored,
      anon_sym_Everything,
      anon_sym_everything,
      anon_sym_Monocolored,
      anon_sym_monocolored,
    ACTIONS(31), 8,
      anon_sym_creatures,
      anon_sym_lands,
      anon_sym_artifacts,
      anon_sym_enchantments,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalkers,
    ACTIONS(29), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(5), 14,
      anon_sym_Flying,
      anon_sym_flying,
      anon_sym_Firststrike,
      anon_sym_firststrike,
      anon_sym_Lifelink,
      anon_sym_lifelink,
      anon_sym_Vigilance,
      anon_sym_vigilance,
      anon_sym_Deathtouch,
      anon_sym_deathtouch,
      anon_sym_Haste,
      anon_sym_haste,
      anon_sym_Visit,
      anon_sym_visit,
  [84] = 12,
    ACTIONS(23), 1,
      anon_sym_have,
    STATE(64), 1,
      sym_keyword_ability,
    STATE(78), 1,
      sym_keyword_ability_protection,
    STATE(112), 1,
      sym_static_ability_effect,
    ACTIONS(21), 2,
      anon_sym_Protection,
      anon_sym_protection,
    STATE(74), 2,
      sym_qualifier_color,
      sym_qualifier_type,
    STATE(109), 2,
      sym_keyword_ability_list,
      sym_protection_qualifier,
    ACTIONS(27), 6,
      anon_sym_creature,
      anon_sym_land,
      anon_sym_artifact,
      anon_sym_enchantment,
      anon_sym_instant,
      anon_sym_planeswalker,
    ACTIONS(33), 6,
      anon_sym_Multicolored,
      anon_sym_multicolored,
      anon_sym_Everything,
      anon_sym_everything,
      anon_sym_Monocolored,
      anon_sym_monocolored,
    ACTIONS(31), 8,
      anon_sym_creatures,
      anon_sym_lands,
      anon_sym_artifacts,
      anon_sym_enchantments,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalkers,
    ACTIONS(29), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(5), 14,
      anon_sym_Flying,
      anon_sym_flying,
      anon_sym_Firststrike,
      anon_sym_firststrike,
      anon_sym_Lifelink,
      anon_sym_lifelink,
      anon_sym_Vigilance,
      anon_sym_vigilance,
      anon_sym_Deathtouch,
      anon_sym_deathtouch,
      anon_sym_Haste,
      anon_sym_haste,
      anon_sym_Visit,
      anon_sym_visit,
  [165] = 11,
    ACTIONS(37), 1,
      anon_sym_TILDE,
    ACTIONS(55), 1,
      anon_sym_with,
    ACTIONS(58), 1,
      sym_any_subtype,
    STATE(4), 2,
      sym_qualifier,
      aux_sym_subject_nontarget_repeat1,
    ACTIONS(35), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
    ACTIONS(46), 3,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(43), 4,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
    STATE(13), 6,
      sym_qualifier_control,
      sym_qualifier_color,
      sym_qualifier_supertype,
      sym_qualifier_type,
      sym_subtype,
      sym_qualifier_pt,
    ACTIONS(52), 8,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
    ACTIONS(49), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(40), 14,
      anon_sym_creature,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
  [243] = 11,
    ACTIONS(63), 1,
      anon_sym_TILDE,
    ACTIONS(75), 1,
      anon_sym_with,
    ACTIONS(77), 1,
      sym_any_subtype,
    STATE(4), 2,
      sym_qualifier,
      aux_sym_subject_nontarget_repeat1,
    ACTIONS(61), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
    ACTIONS(69), 3,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(67), 4,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
    STATE(13), 6,
      sym_qualifier_control,
      sym_qualifier_color,
      sym_qualifier_supertype,
      sym_qualifier_type,
      sym_subtype,
      sym_qualifier_pt,
    ACTIONS(73), 8,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
    ACTIONS(71), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(65), 14,
      anon_sym_creature,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
  [321] = 12,
    ACTIONS(61), 1,
      ts_builtin_sym_end,
    ACTIONS(79), 1,
      anon_sym_have,
    ACTIONS(81), 1,
      anon_sym_TILDE,
    ACTIONS(91), 1,
      anon_sym_with,
    ACTIONS(93), 1,
      sym_any_subtype,
    STATE(7), 2,
      sym_qualifier,
      aux_sym_subject_nontarget_repeat1,
    ACTIONS(85), 3,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(83), 4,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
    STATE(23), 6,
      sym_qualifier_control,
      sym_qualifier_color,
      sym_qualifier_supertype,
      sym_qualifier_type,
      sym_subtype,
      sym_qualifier_pt,
    ACTIONS(89), 8,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
    ACTIONS(87), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(27), 14,
      anon_sym_creature,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
  [400] = 12,
    ACTIONS(35), 1,
      ts_builtin_sym_end,
    ACTIONS(95), 1,
      anon_sym_have,
    ACTIONS(97), 1,
      anon_sym_TILDE,
    ACTIONS(115), 1,
      anon_sym_with,
    ACTIONS(118), 1,
      sym_any_subtype,
    STATE(7), 2,
      sym_qualifier,
      aux_sym_subject_nontarget_repeat1,
    ACTIONS(106), 3,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(103), 4,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
    STATE(23), 6,
      sym_qualifier_control,
      sym_qualifier_color,
      sym_qualifier_supertype,
      sym_qualifier_type,
      sym_subtype,
      sym_qualifier_pt,
    ACTIONS(112), 8,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
    ACTIONS(109), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(100), 14,
      anon_sym_creature,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
  [479] = 4,
    ACTIONS(127), 1,
      anon_sym_anynumberof,
    ACTIONS(123), 4,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(121), 7,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
      anon_sym_Player,
      anon_sym_player,
      anon_sym_Players,
      anon_sym_players,
    ACTIONS(125), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [540] = 10,
    ACTIONS(63), 1,
      anon_sym_TILDE,
    ACTIONS(75), 1,
      anon_sym_with,
    ACTIONS(77), 1,
      sym_any_subtype,
    STATE(5), 2,
      sym_qualifier,
      aux_sym_subject_nontarget_repeat1,
    ACTIONS(69), 3,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(67), 4,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
    STATE(13), 6,
      sym_qualifier_control,
      sym_qualifier_color,
      sym_qualifier_supertype,
      sym_qualifier_type,
      sym_subtype,
      sym_qualifier_pt,
    ACTIONS(73), 8,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
    ACTIONS(71), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(65), 14,
      anon_sym_creature,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
  [613] = 10,
    ACTIONS(81), 1,
      anon_sym_TILDE,
    ACTIONS(91), 1,
      anon_sym_with,
    ACTIONS(93), 1,
      sym_any_subtype,
    STATE(6), 2,
      sym_qualifier,
      aux_sym_subject_nontarget_repeat1,
    ACTIONS(85), 3,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(83), 4,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
    STATE(23), 6,
      sym_qualifier_control,
      sym_qualifier_color,
      sym_qualifier_supertype,
      sym_qualifier_type,
      sym_subtype,
      sym_qualifier_pt,
    ACTIONS(89), 8,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
    ACTIONS(87), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
    ACTIONS(27), 14,
      anon_sym_creature,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
  [686] = 2,
    ACTIONS(129), 8,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(131), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [740] = 2,
    ACTIONS(133), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(135), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [792] = 2,
    ACTIONS(137), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(139), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [844] = 2,
    ACTIONS(141), 6,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(143), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [896] = 2,
    ACTIONS(145), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(147), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [948] = 2,
    ACTIONS(149), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(151), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1000] = 2,
    ACTIONS(153), 6,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(155), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1052] = 2,
    ACTIONS(129), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(131), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1104] = 2,
    ACTIONS(153), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(155), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1156] = 2,
    ACTIONS(141), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(143), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1208] = 2,
    ACTIONS(157), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(159), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1260] = 2,
    ACTIONS(161), 7,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(163), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1312] = 2,
    ACTIONS(137), 5,
      ts_builtin_sym_end,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(139), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1363] = 2,
    ACTIONS(149), 5,
      ts_builtin_sym_end,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(151), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1414] = 2,
    ACTIONS(157), 5,
      ts_builtin_sym_end,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(159), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1465] = 2,
    ACTIONS(133), 5,
      ts_builtin_sym_end,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(135), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1516] = 2,
    ACTIONS(145), 5,
      ts_builtin_sym_end,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(147), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1567] = 2,
    ACTIONS(161), 5,
      ts_builtin_sym_end,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(163), 41,
      anon_sym_have,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1618] = 2,
    ACTIONS(123), 4,
      anon_sym_TILDE,
      anon_sym_youcontrol,
      anon_sym_yourcontrol,
      anon_sym_opponent_SQUOTEscontrol,
    ACTIONS(125), 40,
      anon_sym_creature,
      anon_sym_Untapped,
      anon_sym_untapped,
      anon_sym_Attacking,
      anon_sym_attacking,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
      anon_sym_legendary,
      anon_sym_legendaries,
      anon_sym_basic,
      anon_sym_basics,
      anon_sym_snow,
      anon_sym_snows,
      anon_sym_world,
      anon_sym_worlds,
      anon_sym_creatures,
      anon_sym_land,
      anon_sym_lands,
      anon_sym_artifact,
      anon_sym_artifacts,
      anon_sym_enchantment,
      anon_sym_enchantments,
      anon_sym_instant,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalker,
      anon_sym_planeswalkers,
      anon_sym_with,
      sym_any_subtype,
  [1667] = 8,
    ACTIONS(165), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      aux_sym_line_effect_repeat1,
    STATE(41), 1,
      sym_action_verb,
    STATE(51), 1,
      sym_continuous_tense_qualifier,
    STATE(119), 1,
      sym_effect,
    STATE(120), 2,
      sym_draw_effect,
      sym_one_shot_effect,
    ACTIONS(167), 3,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
    ACTIONS(7), 26,
      anon_sym_Destroy,
      anon_sym_Exile,
      anon_sym_exile,
      anon_sym_destroy,
      anon_sym_Tap,
      anon_sym_tap,
      anon_sym_Untap,
      anon_sym_untap,
      anon_sym_Discard,
      anon_sym_discard,
      anon_sym_Sacrifice,
      anon_sym_sacrifice,
      anon_sym_Create,
      anon_sym_create,
      anon_sym_Counter,
      anon_sym_counter,
      anon_sym_Draw,
      anon_sym_draw,
      anon_sym_Mill,
      anon_sym_mill,
      anon_sym_Scry,
      anon_sym_scry,
      anon_sym_Gain,
      anon_sym_gain,
      anon_sym_Add,
      anon_sym_add,
  [1720] = 8,
    ACTIONS(169), 1,
      ts_builtin_sym_end,
    STATE(31), 1,
      aux_sym_line_effect_repeat1,
    STATE(41), 1,
      sym_action_verb,
    STATE(51), 1,
      sym_continuous_tense_qualifier,
    STATE(119), 1,
      sym_effect,
    STATE(120), 2,
      sym_draw_effect,
      sym_one_shot_effect,
    ACTIONS(171), 3,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
    ACTIONS(174), 26,
      anon_sym_Destroy,
      anon_sym_Exile,
      anon_sym_exile,
      anon_sym_destroy,
      anon_sym_Tap,
      anon_sym_tap,
      anon_sym_Untap,
      anon_sym_untap,
      anon_sym_Discard,
      anon_sym_discard,
      anon_sym_Sacrifice,
      anon_sym_sacrifice,
      anon_sym_Create,
      anon_sym_create,
      anon_sym_Counter,
      anon_sym_counter,
      anon_sym_Draw,
      anon_sym_draw,
      anon_sym_Mill,
      anon_sym_mill,
      anon_sym_Scry,
      anon_sym_scry,
      anon_sym_Gain,
      anon_sym_gain,
      anon_sym_Add,
      anon_sym_add,
  [1773] = 8,
    ACTIONS(15), 1,
      anon_sym_LBRACE,
    ACTIONS(19), 1,
      anon_sym_Pay,
    STATE(41), 1,
      sym_action_verb,
    STATE(49), 1,
      aux_sym_mana_cost_repeat1,
    STATE(76), 1,
      sym_one_shot_effect,
    ACTIONS(177), 2,
      anon_sym_LBRACET_RBRACE,
      anon_sym_LBRACEQ_RBRACE,
    STATE(77), 3,
      sym_mana_cost,
      sym_action_cost,
      sym_cost_life_payment,
    ACTIONS(7), 26,
      anon_sym_Destroy,
      anon_sym_Exile,
      anon_sym_exile,
      anon_sym_destroy,
      anon_sym_Tap,
      anon_sym_tap,
      anon_sym_Untap,
      anon_sym_untap,
      anon_sym_Discard,
      anon_sym_discard,
      anon_sym_Sacrifice,
      anon_sym_sacrifice,
      anon_sym_Create,
      anon_sym_create,
      anon_sym_Counter,
      anon_sym_counter,
      anon_sym_Draw,
      anon_sym_draw,
      anon_sym_Mill,
      anon_sym_mill,
      anon_sym_Scry,
      anon_sym_scry,
      anon_sym_Gain,
      anon_sym_gain,
      anon_sym_Add,
      anon_sym_add,
  [1826] = 6,
    STATE(75), 1,
      sym_protection_qualifier,
    STATE(74), 2,
      sym_qualifier_color,
      sym_qualifier_type,
    ACTIONS(27), 6,
      anon_sym_creature,
      anon_sym_land,
      anon_sym_artifact,
      anon_sym_enchantment,
      anon_sym_instant,
      anon_sym_planeswalker,
    ACTIONS(33), 6,
      anon_sym_Multicolored,
      anon_sym_multicolored,
      anon_sym_Everything,
      anon_sym_everything,
      anon_sym_Monocolored,
      anon_sym_monocolored,
    ACTIONS(31), 8,
      anon_sym_creatures,
      anon_sym_lands,
      anon_sym_artifacts,
      anon_sym_enchantments,
      anon_sym_instants,
      anon_sym_sorcery,
      anon_sym_sorceries,
      anon_sym_planeswalkers,
    ACTIONS(29), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
  [1874] = 6,
    STATE(41), 1,
      sym_action_verb,
    STATE(51), 1,
      sym_continuous_tense_qualifier,
    STATE(128), 1,
      sym_effect,
    STATE(120), 2,
      sym_draw_effect,
      sym_one_shot_effect,
    ACTIONS(167), 3,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
    ACTIONS(7), 26,
      anon_sym_Destroy,
      anon_sym_Exile,
      anon_sym_exile,
      anon_sym_destroy,
      anon_sym_Tap,
      anon_sym_tap,
      anon_sym_Untap,
      anon_sym_untap,
      anon_sym_Discard,
      anon_sym_discard,
      anon_sym_Sacrifice,
      anon_sym_sacrifice,
      anon_sym_Create,
      anon_sym_create,
      anon_sym_Counter,
      anon_sym_counter,
      anon_sym_Draw,
      anon_sym_draw,
      anon_sym_Mill,
      anon_sym_mill,
      anon_sym_Scry,
      anon_sym_scry,
      anon_sym_Gain,
      anon_sym_gain,
      anon_sym_Add,
      anon_sym_add,
  [1921] = 1,
    ACTIONS(169), 30,
      ts_builtin_sym_end,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
      anon_sym_Destroy,
      anon_sym_Exile,
      anon_sym_exile,
      anon_sym_destroy,
      anon_sym_Tap,
      anon_sym_tap,
      anon_sym_Untap,
      anon_sym_untap,
      anon_sym_Discard,
      anon_sym_discard,
      anon_sym_Sacrifice,
      anon_sym_sacrifice,
      anon_sym_Create,
      anon_sym_create,
      anon_sym_Counter,
      anon_sym_counter,
      anon_sym_Draw,
      anon_sym_draw,
      anon_sym_Mill,
      anon_sym_mill,
      anon_sym_Scry,
      anon_sym_scry,
      anon_sym_Gain,
      anon_sym_gain,
      anon_sym_Add,
      anon_sym_add,
  [1954] = 5,
    ACTIONS(181), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    ACTIONS(179), 2,
      anon_sym_A,
      anon_sym_a,
    STATE(98), 2,
      sym_number,
      sym_textual_number,
    ACTIONS(185), 5,
      anon_sym_four,
      anon_sym_six,
      anon_sym_seven,
      anon_sym_eight,
      anon_sym_nine,
    ACTIONS(183), 15,
      anon_sym_one,
      anon_sym_two,
      anon_sym_three,
      anon_sym_five,
      anon_sym_ten,
      anon_sym_eleven,
      anon_sym_twelve,
      anon_sym_thirteen,
      anon_sym_fourteen,
      anon_sym_fifteen,
      anon_sym_sixteen,
      anon_sym_seventeen,
      anon_sym_eighteen,
      anon_sym_nineteen,
      anon_sym_twenty,
  [1990] = 4,
    ACTIONS(181), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    STATE(97), 2,
      sym_number,
      sym_textual_number,
    ACTIONS(185), 5,
      anon_sym_four,
      anon_sym_six,
      anon_sym_seven,
      anon_sym_eight,
      anon_sym_nine,
    ACTIONS(183), 15,
      anon_sym_one,
      anon_sym_two,
      anon_sym_three,
      anon_sym_five,
      anon_sym_ten,
      anon_sym_eleven,
      anon_sym_twelve,
      anon_sym_thirteen,
      anon_sym_fourteen,
      anon_sym_fifteen,
      anon_sym_sixteen,
      anon_sym_seventeen,
      anon_sym_eighteen,
      anon_sym_nineteen,
      anon_sym_twenty,
  [2022] = 4,
    STATE(78), 1,
      sym_keyword_ability_protection,
    STATE(118), 1,
      sym_keyword_ability,
    ACTIONS(21), 2,
      anon_sym_Protection,
      anon_sym_protection,
    ACTIONS(5), 14,
      anon_sym_Flying,
      anon_sym_flying,
      anon_sym_Firststrike,
      anon_sym_firststrike,
      anon_sym_Lifelink,
      anon_sym_lifelink,
      anon_sym_Vigilance,
      anon_sym_vigilance,
      anon_sym_Deathtouch,
      anon_sym_deathtouch,
      anon_sym_Haste,
      anon_sym_haste,
      anon_sym_Visit,
      anon_sym_visit,
  [2049] = 4,
    STATE(78), 1,
      sym_keyword_ability_protection,
    STATE(79), 1,
      sym_keyword_ability,
    ACTIONS(21), 2,
      anon_sym_Protection,
      anon_sym_protection,
    ACTIONS(5), 14,
      anon_sym_Flying,
      anon_sym_flying,
      anon_sym_Firststrike,
      anon_sym_firststrike,
      anon_sym_Lifelink,
      anon_sym_lifelink,
      anon_sym_Vigilance,
      anon_sym_vigilance,
      anon_sym_Deathtouch,
      anon_sym_deathtouch,
      anon_sym_Haste,
      anon_sym_haste,
      anon_sym_Visit,
      anon_sym_visit,
  [2076] = 9,
    ACTIONS(187), 1,
      anon_sym_S,
    ACTIONS(191), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    ACTIONS(193), 1,
      anon_sym_X,
    ACTIONS(195), 1,
      sym_colorless_mana_cost_symbol,
    ACTIONS(197), 1,
      anon_sym_2,
    STATE(99), 1,
      sym_mana_color,
    STATE(104), 1,
      sym_mana_cost_symbol,
    ACTIONS(189), 5,
      anon_sym_W,
      anon_sym_U,
      anon_sym_B,
      anon_sym_R,
      anon_sym_G,
    STATE(107), 5,
      sym_plain_mana_cost_symbol,
      sym_generic_mana_cost_symbol,
      sym_snow_mana_cost_symbol,
      sym_hybrid_mana_cost_symbol,
      sym_phyrexian_mana_cost_symbol,
  [2112] = 10,
    ACTIONS(181), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    ACTIONS(201), 1,
      anon_sym_upto,
    ACTIONS(203), 1,
      anon_sym_another,
    STATE(9), 1,
      sym_non_finite_quantity,
    STATE(70), 1,
      sym_subject,
    STATE(80), 1,
      sym_finite_quantity,
    STATE(82), 1,
      sym_number,
    ACTIONS(199), 2,
      anon_sym_Target,
      anon_sym_target,
    STATE(68), 2,
      sym_subject_target,
      sym_subject_nontarget,
    ACTIONS(13), 4,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
      anon_sym_anynumberof,
  [2148] = 4,
    ACTIONS(93), 1,
      sym_any_subtype,
    STATE(54), 1,
      sym_subtype,
    STATE(100), 1,
      sym_qualifier_color,
    ACTIONS(205), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
  [2172] = 4,
    ACTIONS(93), 1,
      sym_any_subtype,
    STATE(62), 1,
      sym_subtype,
    STATE(81), 1,
      sym_qualifier_color,
    ACTIONS(205), 12,
      anon_sym_White,
      anon_sym_white,
      anon_sym_Blue,
      anon_sym_blue,
      anon_sym_Black,
      anon_sym_black,
      anon_sym_Red,
      anon_sym_red,
      anon_sym_Green,
      anon_sym_green,
      anon_sym_Colorless,
      anon_sym_colorless,
  [2196] = 6,
    STATE(10), 1,
      sym_non_finite_quantity,
    STATE(88), 1,
      sym_subject_nontarget,
    STATE(89), 1,
      sym_player,
    ACTIONS(207), 2,
      anon_sym_Player,
      anon_sym_player,
    ACTIONS(209), 2,
      anon_sym_Players,
      anon_sym_players,
    ACTIONS(13), 4,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
      anon_sym_anynumberof,
  [2220] = 1,
    ACTIONS(211), 9,
      anon_sym_all,
      anon_sym_each,
      anon_sym_every,
      anon_sym_Target,
      anon_sym_target,
      anon_sym_upto,
      anon_sym_another,
      anon_sym_anynumberof,
      aux_sym_generic_mana_cost_symbol_token1,
  [2232] = 3,
    ACTIONS(213), 1,
      anon_sym_P,
    STATE(125), 1,
      sym_mana_color,
    ACTIONS(189), 5,
      anon_sym_W,
      anon_sym_U,
      anon_sym_B,
      anon_sym_R,
      anon_sym_G,
  [2246] = 1,
    ACTIONS(133), 6,
      anon_sym_Card,
      anon_sym_card,
      anon_sym_Target,
      anon_sym_target,
      anon_sym_SLASH,
      anon_sym_life,
  [2255] = 2,
    STATE(125), 1,
      sym_mana_color,
    ACTIONS(189), 5,
      anon_sym_W,
      anon_sym_U,
      anon_sym_B,
      anon_sym_R,
      anon_sym_G,
  [2266] = 3,
    ACTIONS(217), 1,
      anon_sym_LBRACE,
    STATE(50), 1,
      aux_sym_mana_cost_repeat1,
    ACTIONS(215), 3,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_COLON,
  [2278] = 3,
    ACTIONS(221), 1,
      anon_sym_LBRACE,
    STATE(50), 1,
      aux_sym_mana_cost_repeat1,
    ACTIONS(219), 3,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_COLON,
  [2290] = 3,
    STATE(89), 1,
      sym_player,
    ACTIONS(207), 2,
      anon_sym_Player,
      anon_sym_player,
    ACTIONS(209), 2,
      anon_sym_Players,
      anon_sym_players,
  [2302] = 1,
    ACTIONS(224), 4,
      ts_builtin_sym_end,
      anon_sym_COMMA,
      anon_sym_COLON,
      anon_sym_LBRACE,
  [2309] = 2,
    ACTIONS(121), 2,
      anon_sym_Player,
      anon_sym_player,
    ACTIONS(127), 2,
      anon_sym_Players,
      anon_sym_players,
  [2318] = 1,
    ACTIONS(226), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
  [2324] = 2,
    ACTIONS(228), 1,
      anon_sym_DOT,
    ACTIONS(230), 2,
      anon_sym_S,
      anon_sym_s,
  [2332] = 1,
    ACTIONS(232), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
  [2338] = 2,
    ACTIONS(234), 1,
      anon_sym_DOT,
    ACTIONS(236), 2,
      anon_sym_S,
      anon_sym_s,
  [2346] = 2,
    STATE(24), 1,
      sym_pt_modifier,
    ACTIONS(238), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2354] = 3,
    ACTIONS(240), 1,
      anon_sym_COMMA,
    ACTIONS(242), 1,
      anon_sym_COLON,
    STATE(67), 1,
      aux_sym_cost_repeat1,
  [2364] = 3,
    ACTIONS(244), 1,
      ts_builtin_sym_end,
    ACTIONS(246), 1,
      anon_sym_COMMA,
    STATE(60), 1,
      aux_sym_keyword_ability_list_repeat1,
  [2374] = 2,
    ACTIONS(251), 1,
      anon_sym_DOT,
    ACTIONS(249), 2,
      anon_sym_COMMA,
      anon_sym_COLON,
  [2382] = 1,
    ACTIONS(253), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
  [2388] = 2,
    STATE(16), 1,
      sym_pt_modifier,
    ACTIONS(255), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2396] = 3,
    ACTIONS(257), 1,
      ts_builtin_sym_end,
    ACTIONS(259), 1,
      anon_sym_COMMA,
    STATE(65), 1,
      aux_sym_keyword_ability_list_repeat1,
  [2406] = 3,
    ACTIONS(259), 1,
      anon_sym_COMMA,
    ACTIONS(261), 1,
      ts_builtin_sym_end,
    STATE(60), 1,
      aux_sym_keyword_ability_list_repeat1,
  [2416] = 3,
    ACTIONS(217), 1,
      anon_sym_LBRACE,
    STATE(49), 1,
      aux_sym_mana_cost_repeat1,
    STATE(127), 1,
      sym_mana_cost,
  [2426] = 3,
    ACTIONS(263), 1,
      anon_sym_COMMA,
    ACTIONS(266), 1,
      anon_sym_COLON,
    STATE(67), 1,
      aux_sym_cost_repeat1,
  [2436] = 1,
    ACTIONS(268), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
  [2442] = 3,
    ACTIONS(240), 1,
      anon_sym_COMMA,
    ACTIONS(270), 1,
      anon_sym_COLON,
    STATE(59), 1,
      aux_sym_cost_repeat1,
  [2452] = 1,
    ACTIONS(272), 3,
      anon_sym_COMMA,
      anon_sym_DOT,
      anon_sym_COLON,
  [2458] = 3,
    ACTIONS(240), 1,
      anon_sym_COMMA,
    ACTIONS(274), 1,
      anon_sym_COLON,
    STATE(59), 1,
      aux_sym_cost_repeat1,
  [2468] = 1,
    ACTIONS(276), 2,
      anon_sym_Card,
      anon_sym_card,
  [2473] = 2,
    ACTIONS(181), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    STATE(102), 1,
      sym_number,
  [2480] = 1,
    ACTIONS(278), 2,
      ts_builtin_sym_end,
      anon_sym_COMMA,
  [2485] = 1,
    ACTIONS(280), 2,
      ts_builtin_sym_end,
      anon_sym_COMMA,
  [2490] = 1,
    ACTIONS(249), 2,
      anon_sym_COMMA,
      anon_sym_COLON,
  [2495] = 1,
    ACTIONS(266), 2,
      anon_sym_COMMA,
      anon_sym_COLON,
  [2500] = 1,
    ACTIONS(282), 2,
      ts_builtin_sym_end,
      anon_sym_COMMA,
  [2505] = 1,
    ACTIONS(244), 2,
      ts_builtin_sym_end,
      anon_sym_COMMA,
  [2510] = 1,
    ACTIONS(284), 2,
      anon_sym_Target,
      anon_sym_target,
  [2515] = 2,
    ACTIONS(286), 1,
      sym_any_subtype,
    STATE(54), 1,
      sym_subtype,
  [2522] = 1,
    ACTIONS(288), 2,
      anon_sym_Target,
      anon_sym_target,
  [2527] = 2,
    ACTIONS(181), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    STATE(129), 1,
      sym_number,
  [2534] = 1,
    ACTIONS(290), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2539] = 1,
    ACTIONS(292), 2,
      anon_sym_Target,
      anon_sym_target,
  [2544] = 2,
    ACTIONS(181), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    STATE(85), 1,
      sym_number,
  [2551] = 1,
    ACTIONS(294), 2,
      anon_sym_PLUS,
      anon_sym_DASH,
  [2556] = 1,
    ACTIONS(296), 2,
      ts_builtin_sym_end,
      anon_sym_have,
  [2561] = 1,
    ACTIONS(298), 2,
      anon_sym_Draws,
      anon_sym_draws,
  [2566] = 2,
    ACTIONS(300), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    STATE(15), 1,
      sym_number,
  [2573] = 2,
    ACTIONS(181), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    STATE(103), 1,
      sym_number,
  [2580] = 2,
    ACTIONS(302), 1,
      ts_builtin_sym_end,
    ACTIONS(304), 1,
      anon_sym_have,
  [2587] = 1,
    ACTIONS(306), 2,
      ts_builtin_sym_end,
      anon_sym_have,
  [2592] = 1,
    ACTIONS(308), 2,
      anon_sym_SLASH,
      anon_sym_RBRACE,
  [2597] = 2,
    ACTIONS(310), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    STATE(27), 1,
      sym_number,
  [2604] = 1,
    ACTIONS(312), 2,
      anon_sym_Draws,
      anon_sym_draws,
  [2609] = 1,
    ACTIONS(314), 2,
      anon_sym_Card,
      anon_sym_card,
  [2614] = 1,
    ACTIONS(316), 2,
      anon_sym_Card,
      anon_sym_card,
  [2619] = 2,
    ACTIONS(318), 1,
      anon_sym_SLASH,
    ACTIONS(320), 1,
      anon_sym_RBRACE,
  [2626] = 2,
    ACTIONS(286), 1,
      sym_any_subtype,
    STATE(56), 1,
      sym_subtype,
  [2633] = 1,
    ACTIONS(322), 2,
      anon_sym_COMMA,
      anon_sym_COLON,
  [2638] = 1,
    ACTIONS(324), 1,
      anon_sym_SLASH,
  [2642] = 1,
    ACTIONS(326), 1,
      anon_sym_life,
  [2646] = 1,
    ACTIONS(328), 1,
      anon_sym_RBRACE,
  [2650] = 1,
    ACTIONS(330), 1,
      ts_builtin_sym_end,
  [2654] = 1,
    ACTIONS(332), 1,
      anon_sym_SLASH,
  [2658] = 1,
    ACTIONS(334), 1,
      anon_sym_RBRACE,
  [2662] = 1,
    ACTIONS(234), 1,
      anon_sym_DOT,
  [2666] = 1,
    ACTIONS(336), 1,
      ts_builtin_sym_end,
  [2670] = 1,
    ACTIONS(338), 1,
      anon_sym_DOT,
  [2674] = 1,
    ACTIONS(340), 1,
      anon_sym_RBRACE,
  [2678] = 1,
    ACTIONS(342), 1,
      ts_builtin_sym_end,
  [2682] = 1,
    ACTIONS(344), 1,
      anon_sym_RBRACE,
  [2686] = 1,
    ACTIONS(346), 1,
      ts_builtin_sym_end,
  [2690] = 1,
    ACTIONS(348), 1,
      anon_sym_Add,
  [2694] = 1,
    ACTIONS(153), 1,
      sym_any_subtype,
  [2698] = 1,
    ACTIONS(350), 1,
      anon_sym_COLON,
  [2702] = 1,
    ACTIONS(352), 1,
      ts_builtin_sym_end,
  [2706] = 1,
    ACTIONS(354), 1,
      anon_sym_DOT,
  [2710] = 1,
    ACTIONS(251), 1,
      anon_sym_DOT,
  [2714] = 1,
    ACTIONS(356), 1,
      anon_sym_RBRACE,
  [2718] = 1,
    ACTIONS(358), 1,
      ts_builtin_sym_end,
  [2722] = 1,
    ACTIONS(360), 1,
      ts_builtin_sym_end,
  [2726] = 1,
    ACTIONS(362), 1,
      anon_sym_from,
  [2730] = 1,
    ACTIONS(364), 1,
      anon_sym_RBRACE,
  [2734] = 1,
    ACTIONS(366), 1,
      anon_sym_can_SQUOTEtbeblocked,
  [2738] = 1,
    ACTIONS(368), 1,
      ts_builtin_sym_end,
  [2742] = 1,
    ACTIONS(370), 1,
      anon_sym_DOT,
  [2746] = 1,
    ACTIONS(372), 1,
      anon_sym_SLASH,
  [2750] = 1,
    ACTIONS(374), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 84,
  [SMALL_STATE(4)] = 165,
  [SMALL_STATE(5)] = 243,
  [SMALL_STATE(6)] = 321,
  [SMALL_STATE(7)] = 400,
  [SMALL_STATE(8)] = 479,
  [SMALL_STATE(9)] = 540,
  [SMALL_STATE(10)] = 613,
  [SMALL_STATE(11)] = 686,
  [SMALL_STATE(12)] = 740,
  [SMALL_STATE(13)] = 792,
  [SMALL_STATE(14)] = 844,
  [SMALL_STATE(15)] = 896,
  [SMALL_STATE(16)] = 948,
  [SMALL_STATE(17)] = 1000,
  [SMALL_STATE(18)] = 1052,
  [SMALL_STATE(19)] = 1104,
  [SMALL_STATE(20)] = 1156,
  [SMALL_STATE(21)] = 1208,
  [SMALL_STATE(22)] = 1260,
  [SMALL_STATE(23)] = 1312,
  [SMALL_STATE(24)] = 1363,
  [SMALL_STATE(25)] = 1414,
  [SMALL_STATE(26)] = 1465,
  [SMALL_STATE(27)] = 1516,
  [SMALL_STATE(28)] = 1567,
  [SMALL_STATE(29)] = 1618,
  [SMALL_STATE(30)] = 1667,
  [SMALL_STATE(31)] = 1720,
  [SMALL_STATE(32)] = 1773,
  [SMALL_STATE(33)] = 1826,
  [SMALL_STATE(34)] = 1874,
  [SMALL_STATE(35)] = 1921,
  [SMALL_STATE(36)] = 1954,
  [SMALL_STATE(37)] = 1990,
  [SMALL_STATE(38)] = 2022,
  [SMALL_STATE(39)] = 2049,
  [SMALL_STATE(40)] = 2076,
  [SMALL_STATE(41)] = 2112,
  [SMALL_STATE(42)] = 2148,
  [SMALL_STATE(43)] = 2172,
  [SMALL_STATE(44)] = 2196,
  [SMALL_STATE(45)] = 2220,
  [SMALL_STATE(46)] = 2232,
  [SMALL_STATE(47)] = 2246,
  [SMALL_STATE(48)] = 2255,
  [SMALL_STATE(49)] = 2266,
  [SMALL_STATE(50)] = 2278,
  [SMALL_STATE(51)] = 2290,
  [SMALL_STATE(52)] = 2302,
  [SMALL_STATE(53)] = 2309,
  [SMALL_STATE(54)] = 2318,
  [SMALL_STATE(55)] = 2324,
  [SMALL_STATE(56)] = 2332,
  [SMALL_STATE(57)] = 2338,
  [SMALL_STATE(58)] = 2346,
  [SMALL_STATE(59)] = 2354,
  [SMALL_STATE(60)] = 2364,
  [SMALL_STATE(61)] = 2374,
  [SMALL_STATE(62)] = 2382,
  [SMALL_STATE(63)] = 2388,
  [SMALL_STATE(64)] = 2396,
  [SMALL_STATE(65)] = 2406,
  [SMALL_STATE(66)] = 2416,
  [SMALL_STATE(67)] = 2426,
  [SMALL_STATE(68)] = 2436,
  [SMALL_STATE(69)] = 2442,
  [SMALL_STATE(70)] = 2452,
  [SMALL_STATE(71)] = 2458,
  [SMALL_STATE(72)] = 2468,
  [SMALL_STATE(73)] = 2473,
  [SMALL_STATE(74)] = 2480,
  [SMALL_STATE(75)] = 2485,
  [SMALL_STATE(76)] = 2490,
  [SMALL_STATE(77)] = 2495,
  [SMALL_STATE(78)] = 2500,
  [SMALL_STATE(79)] = 2505,
  [SMALL_STATE(80)] = 2510,
  [SMALL_STATE(81)] = 2515,
  [SMALL_STATE(82)] = 2522,
  [SMALL_STATE(83)] = 2527,
  [SMALL_STATE(84)] = 2534,
  [SMALL_STATE(85)] = 2539,
  [SMALL_STATE(86)] = 2544,
  [SMALL_STATE(87)] = 2551,
  [SMALL_STATE(88)] = 2556,
  [SMALL_STATE(89)] = 2561,
  [SMALL_STATE(90)] = 2566,
  [SMALL_STATE(91)] = 2573,
  [SMALL_STATE(92)] = 2580,
  [SMALL_STATE(93)] = 2587,
  [SMALL_STATE(94)] = 2592,
  [SMALL_STATE(95)] = 2597,
  [SMALL_STATE(96)] = 2604,
  [SMALL_STATE(97)] = 2609,
  [SMALL_STATE(98)] = 2614,
  [SMALL_STATE(99)] = 2619,
  [SMALL_STATE(100)] = 2626,
  [SMALL_STATE(101)] = 2633,
  [SMALL_STATE(102)] = 2638,
  [SMALL_STATE(103)] = 2642,
  [SMALL_STATE(104)] = 2646,
  [SMALL_STATE(105)] = 2650,
  [SMALL_STATE(106)] = 2654,
  [SMALL_STATE(107)] = 2658,
  [SMALL_STATE(108)] = 2662,
  [SMALL_STATE(109)] = 2666,
  [SMALL_STATE(110)] = 2670,
  [SMALL_STATE(111)] = 2674,
  [SMALL_STATE(112)] = 2678,
  [SMALL_STATE(113)] = 2682,
  [SMALL_STATE(114)] = 2686,
  [SMALL_STATE(115)] = 2690,
  [SMALL_STATE(116)] = 2694,
  [SMALL_STATE(117)] = 2698,
  [SMALL_STATE(118)] = 2702,
  [SMALL_STATE(119)] = 2706,
  [SMALL_STATE(120)] = 2710,
  [SMALL_STATE(121)] = 2714,
  [SMALL_STATE(122)] = 2718,
  [SMALL_STATE(123)] = 2722,
  [SMALL_STATE(124)] = 2726,
  [SMALL_STATE(125)] = 2730,
  [SMALL_STATE(126)] = 2734,
  [SMALL_STATE(127)] = 2738,
  [SMALL_STATE(128)] = 2742,
  [SMALL_STATE(129)] = 2746,
  [SMALL_STATE(130)] = 2750,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(78),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(71),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(126),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(40),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(69),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(91),
  [21] = {.entry = {.count = 1, .reusable = true}}, SHIFT(124),
  [23] = {.entry = {.count = 1, .reusable = true}}, SHIFT(38),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [27] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [31] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [33] = {.entry = {.count = 1, .reusable = true}}, SHIFT(74),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2),
  [37] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(13),
  [40] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(20),
  [43] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(13),
  [46] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(22),
  [49] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(19),
  [52] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(21),
  [55] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(63),
  [58] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(18),
  [61] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subject_nontarget, 2),
  [63] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [65] = {.entry = {.count = 1, .reusable = false}}, SHIFT(20),
  [67] = {.entry = {.count = 1, .reusable = false}}, SHIFT(13),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [71] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [73] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(63),
  [77] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_subject_nontarget, 2),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [83] = {.entry = {.count = 1, .reusable = false}}, SHIFT(23),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [87] = {.entry = {.count = 1, .reusable = false}}, SHIFT(17),
  [89] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [91] = {.entry = {.count = 1, .reusable = false}}, SHIFT(58),
  [93] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [95] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2),
  [97] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(23),
  [100] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(14),
  [103] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(23),
  [106] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(28),
  [109] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(17),
  [112] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(25),
  [115] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(58),
  [118] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_subject_nontarget_repeat1, 2), SHIFT_REPEAT(11),
  [121] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_continuous_tense_qualifier, 1),
  [123] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_non_finite_quantity, 1),
  [125] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_non_finite_quantity, 1),
  [127] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_continuous_tense_qualifier, 1),
  [129] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subtype, 1),
  [131] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_subtype, 1),
  [133] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_number, 1),
  [135] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_number, 1),
  [137] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualifier, 1),
  [139] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_qualifier, 1),
  [141] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualifier_type, 1),
  [143] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_qualifier_type, 1),
  [145] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_pt_modifier, 5),
  [147] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_pt_modifier, 5),
  [149] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualifier_pt, 2),
  [151] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_qualifier_pt, 2),
  [153] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualifier_color, 1),
  [155] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_qualifier_color, 1),
  [157] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualifier_supertype, 1),
  [159] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_qualifier_supertype, 1),
  [161] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_qualifier_control, 1),
  [163] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_qualifier_control, 1),
  [165] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_effect, 1),
  [167] = {.entry = {.count = 1, .reusable = true}}, SHIFT(53),
  [169] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_line_effect_repeat1, 2),
  [171] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_line_effect_repeat1, 2), SHIFT_REPEAT(53),
  [174] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_line_effect_repeat1, 2), SHIFT_REPEAT(45),
  [177] = {.entry = {.count = 1, .reusable = true}}, SHIFT(77),
  [179] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [181] = {.entry = {.count = 1, .reusable = true}}, SHIFT(47),
  [183] = {.entry = {.count = 1, .reusable = true}}, SHIFT(72),
  [185] = {.entry = {.count = 1, .reusable = false}}, SHIFT(72),
  [187] = {.entry = {.count = 1, .reusable = true}}, SHIFT(113),
  [189] = {.entry = {.count = 1, .reusable = true}}, SHIFT(94),
  [191] = {.entry = {.count = 1, .reusable = false}}, SHIFT(111),
  [193] = {.entry = {.count = 1, .reusable = true}}, SHIFT(111),
  [195] = {.entry = {.count = 1, .reusable = true}}, SHIFT(107),
  [197] = {.entry = {.count = 1, .reusable = false}}, SHIFT(106),
  [199] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [201] = {.entry = {.count = 1, .reusable = true}}, SHIFT(86),
  [203] = {.entry = {.count = 1, .reusable = true}}, SHIFT(82),
  [205] = {.entry = {.count = 1, .reusable = false}}, SHIFT(116),
  [207] = {.entry = {.count = 1, .reusable = false}}, SHIFT(96),
  [209] = {.entry = {.count = 1, .reusable = true}}, SHIFT(96),
  [211] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_verb, 1),
  [213] = {.entry = {.count = 1, .reusable = true}}, SHIFT(121),
  [215] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_cost, 1),
  [217] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [219] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_mana_cost_repeat1, 2),
  [221] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_mana_cost_repeat1, 2), SHIFT_REPEAT(40),
  [224] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_mana_cost_repeat1, 3),
  [226] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subject_target, 3),
  [228] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_draw_effect, 5),
  [230] = {.entry = {.count = 1, .reusable = true}}, SHIFT(108),
  [232] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subject_target, 4),
  [234] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_draw_effect, 6),
  [236] = {.entry = {.count = 1, .reusable = true}}, SHIFT(110),
  [238] = {.entry = {.count = 1, .reusable = true}}, SHIFT(73),
  [240] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [242] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cost, 2),
  [244] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_keyword_ability_list_repeat1, 2),
  [246] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_keyword_ability_list_repeat1, 2), SHIFT_REPEAT(39),
  [249] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_action_cost, 1),
  [251] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_effect, 1),
  [253] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subject_target, 2),
  [255] = {.entry = {.count = 1, .reusable = true}}, SHIFT(83),
  [257] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyword_ability_list, 1),
  [259] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [261] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyword_ability_list, 2),
  [263] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_cost_repeat1, 2), SHIFT_REPEAT(32),
  [266] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_cost_repeat1, 2),
  [268] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_subject, 1),
  [270] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cost, 1),
  [272] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_one_shot_effect, 2),
  [274] = {.entry = {.count = 1, .reusable = true}}, SHIFT(115),
  [276] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_textual_number, 1),
  [278] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_protection_qualifier, 1),
  [280] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyword_ability_protection, 3),
  [282] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_keyword_ability, 1),
  [284] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [286] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [288] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_finite_quantity, 1),
  [290] = {.entry = {.count = 1, .reusable = true}}, SHIFT(90),
  [292] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_finite_quantity, 2),
  [294] = {.entry = {.count = 1, .reusable = true}}, SHIFT(95),
  [296] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_static_ability_subject, 2),
  [298] = {.entry = {.count = 1, .reusable = true}}, SHIFT(36),
  [300] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [302] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_static_ability, 1),
  [304] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [306] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_static_ability_subject, 1),
  [308] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_color, 1),
  [310] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [312] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_player, 1),
  [314] = {.entry = {.count = 1, .reusable = true}}, SHIFT(57),
  [316] = {.entry = {.count = 1, .reusable = true}}, SHIFT(55),
  [318] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [320] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_plain_mana_cost_symbol, 1),
  [322] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_cost_life_payment, 3),
  [324] = {.entry = {.count = 1, .reusable = true}}, SHIFT(87),
  [326] = {.entry = {.count = 1, .reusable = true}}, SHIFT(101),
  [328] = {.entry = {.count = 1, .reusable = true}}, SHIFT(52),
  [330] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_activated_ability, 4),
  [332] = {.entry = {.count = 1, .reusable = true}}, SHIFT(48),
  [334] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_cost_symbol, 1),
  [336] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_static_ability_effect, 1),
  [338] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_draw_effect, 7),
  [340] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_generic_mana_cost_symbol, 1),
  [342] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_static_ability, 4),
  [344] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_snow_mana_cost_symbol, 1),
  [346] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unblockable_ability, 2),
  [348] = {.entry = {.count = 1, .reusable = true}}, SHIFT(66),
  [350] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [352] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_static_ability_effect, 2),
  [354] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [356] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_phyrexian_mana_cost_symbol, 3),
  [358] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_rules_line, 1),
  [360] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [362] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [364] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hybrid_mana_cost_symbol, 3),
  [366] = {.entry = {.count = 1, .reusable = true}}, SHIFT(114),
  [368] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_ability, 4),
  [370] = {.entry = {.count = 1, .reusable = true}}, SHIFT(105),
  [372] = {.entry = {.count = 1, .reusable = true}}, SHIFT(84),
  [374] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_line_static_ability, 3),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_mtg_oracle(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
