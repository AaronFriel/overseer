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
#define STATE_COUNT 20
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 159
#define ALIAS_COUNT 0
#define TOKEN_COUNT 149
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum {
  anon_sym_Add = 1,
  anon_sym_COMMA = 2,
  anon_sym_DOT = 3,
  anon_sym_have = 4,
  anon_sym_hexprooffrom = 5,
  anon_sym_all = 6,
  anon_sym_each = 7,
  anon_sym_every = 8,
  anon_sym_Draws = 9,
  anon_sym_draws = 10,
  anon_sym_A = 11,
  anon_sym_a = 12,
  anon_sym_Card = 13,
  anon_sym_card = 14,
  anon_sym_S = 15,
  anon_sym_s = 16,
  anon_sym_COLON = 17,
  anon_sym_Flying = 18,
  anon_sym_flying = 19,
  anon_sym_Firststrike = 20,
  anon_sym_firststrike = 21,
  anon_sym_Lifelink = 22,
  anon_sym_lifelink = 23,
  anon_sym_Vigilance = 24,
  anon_sym_vigilance = 25,
  anon_sym_Deathtouch = 26,
  anon_sym_deathtouch = 27,
  anon_sym_Haste = 28,
  anon_sym_haste = 29,
  anon_sym_Visit = 30,
  anon_sym_visit = 31,
  anon_sym_get = 32,
  anon_sym_Destroy = 33,
  anon_sym_Exile = 34,
  anon_sym_exile = 35,
  anon_sym_destroy = 36,
  anon_sym_Tap = 37,
  anon_sym_tap = 38,
  anon_sym_Untap = 39,
  anon_sym_untap = 40,
  anon_sym_Discard = 41,
  anon_sym_discard = 42,
  anon_sym_Sacrifice = 43,
  anon_sym_sacrifice = 44,
  anon_sym_Create = 45,
  anon_sym_create = 46,
  anon_sym_Counter = 47,
  anon_sym_counter = 48,
  anon_sym_Draw = 49,
  anon_sym_draw = 50,
  anon_sym_Mill = 51,
  anon_sym_mill = 52,
  anon_sym_Scry = 53,
  anon_sym_scry = 54,
  anon_sym_Gain = 55,
  anon_sym_gain = 56,
  anon_sym_add = 57,
  anon_sym_TILDE = 58,
  anon_sym_can_SQUOTEtbeblocked = 59,
  anon_sym_Player = 60,
  anon_sym_player = 61,
  anon_sym_Players = 62,
  anon_sym_players = 63,
  anon_sym_Target = 64,
  anon_sym_target = 65,
  anon_sym_creature = 66,
  anon_sym_upto = 67,
  anon_sym_Untapped = 68,
  anon_sym_untapped = 69,
  anon_sym_youcontrol = 70,
  anon_sym_yourcontrol = 71,
  anon_sym_opponent_SQUOTEscontrol = 72,
  anon_sym_W = 73,
  anon_sym_U = 74,
  anon_sym_B = 75,
  anon_sym_R = 76,
  anon_sym_G = 77,
  aux_sym_generic_mana_cost_symbol_token1 = 78,
  anon_sym_X = 79,
  sym_colorless_mana_cost_symbol = 80,
  anon_sym_2 = 81,
  anon_sym_SLASH = 82,
  anon_sym_P = 83,
  anon_sym_LBRACE = 84,
  anon_sym_RBRACE = 85,
  anon_sym_LBRACET_RBRACE = 86,
  anon_sym_LBRACEQ_RBRACE = 87,
  anon_sym_Pay = 88,
  anon_sym_life = 89,
  anon_sym_White = 90,
  anon_sym_white = 91,
  anon_sym_Blue = 92,
  anon_sym_blue = 93,
  anon_sym_Black = 94,
  anon_sym_black = 95,
  anon_sym_Red = 96,
  anon_sym_red = 97,
  anon_sym_Green = 98,
  anon_sym_green = 99,
  anon_sym_Colorless = 100,
  anon_sym_colorless = 101,
  anon_sym_legendary = 102,
  anon_sym_legendaries = 103,
  anon_sym_basic = 104,
  anon_sym_snow = 105,
  anon_sym_world = 106,
  anon_sym_land = 107,
  anon_sym_artifact = 108,
  anon_sym_enchantment = 109,
  anon_sym_instant = 110,
  anon_sym_sorcery = 111,
  anon_sym_sorceries = 112,
  anon_sym_planeswalker = 113,
  anon_sym_Hexproof = 114,
  anon_sym_hexproof = 115,
  anon_sym_Tapped = 116,
  anon_sym_tapped = 117,
  anon_sym_with = 118,
  anon_sym_PLUS = 119,
  anon_sym_DASH = 120,
  anon_sym_Protection = 121,
  anon_sym_protection = 122,
  anon_sym_from = 123,
  anon_sym_Multicolored = 124,
  anon_sym_multicolored = 125,
  anon_sym_Everything = 126,
  anon_sym_Monocolored = 127,
  anon_sym_monocolored = 128,
  anon_sym_one = 129,
  anon_sym_two = 130,
  anon_sym_three = 131,
  anon_sym_four = 132,
  anon_sym_five = 133,
  anon_sym_six = 134,
  anon_sym_seven = 135,
  anon_sym_eight = 136,
  anon_sym_nine = 137,
  anon_sym_ten = 138,
  anon_sym_eleven = 139,
  anon_sym_twelve = 140,
  anon_sym_thirteen = 141,
  anon_sym_fourteen = 142,
  anon_sym_fifteen = 143,
  anon_sym_sixteen = 144,
  anon_sym_seventeen = 145,
  anon_sym_eighteen = 146,
  anon_sym_nineteen = 147,
  anon_sym_twenty = 148,
  sym_mana_ability = 149,
  sym_mana_color = 150,
  sym_plain_mana_cost_symbol = 151,
  sym_generic_mana_cost_symbol = 152,
  sym_snow_mana_cost_symbol = 153,
  sym_hybrid_mana_cost_symbol = 154,
  sym_phyrexian_mana_cost_symbol = 155,
  sym_mana_cost_symbol = 156,
  sym_mana_cost = 157,
  aux_sym_mana_cost_repeat1 = 158,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_Add] = "Add",
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
  [anon_sym_add] = "add",
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
  [anon_sym_Untapped] = "Untapped",
  [anon_sym_untapped] = "untapped",
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
  [anon_sym_LBRACET_RBRACE] = "{T}",
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
  [anon_sym_snow] = "snow",
  [anon_sym_world] = "world",
  [anon_sym_land] = "land",
  [anon_sym_artifact] = "artifact",
  [anon_sym_enchantment] = "enchantment",
  [anon_sym_instant] = "instant",
  [anon_sym_sorcery] = "sorcery",
  [anon_sym_sorceries] = "sorceries",
  [anon_sym_planeswalker] = "planeswalker",
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
  [sym_mana_ability] = "mana_ability",
  [sym_mana_color] = "mana_color",
  [sym_plain_mana_cost_symbol] = "plain_mana_cost_symbol",
  [sym_generic_mana_cost_symbol] = "generic_mana_cost_symbol",
  [sym_snow_mana_cost_symbol] = "snow_mana_cost_symbol",
  [sym_hybrid_mana_cost_symbol] = "hybrid_mana_cost_symbol",
  [sym_phyrexian_mana_cost_symbol] = "phyrexian_mana_cost_symbol",
  [sym_mana_cost_symbol] = "mana_cost_symbol",
  [sym_mana_cost] = "mana_cost",
  [aux_sym_mana_cost_repeat1] = "mana_cost_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_Add] = anon_sym_Add,
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
  [anon_sym_add] = anon_sym_add,
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
  [anon_sym_Untapped] = anon_sym_Untapped,
  [anon_sym_untapped] = anon_sym_untapped,
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
  [anon_sym_LBRACET_RBRACE] = anon_sym_LBRACET_RBRACE,
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
  [anon_sym_snow] = anon_sym_snow,
  [anon_sym_world] = anon_sym_world,
  [anon_sym_land] = anon_sym_land,
  [anon_sym_artifact] = anon_sym_artifact,
  [anon_sym_enchantment] = anon_sym_enchantment,
  [anon_sym_instant] = anon_sym_instant,
  [anon_sym_sorcery] = anon_sym_sorcery,
  [anon_sym_sorceries] = anon_sym_sorceries,
  [anon_sym_planeswalker] = anon_sym_planeswalker,
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
  [sym_mana_ability] = sym_mana_ability,
  [sym_mana_color] = sym_mana_color,
  [sym_plain_mana_cost_symbol] = sym_plain_mana_cost_symbol,
  [sym_generic_mana_cost_symbol] = sym_generic_mana_cost_symbol,
  [sym_snow_mana_cost_symbol] = sym_snow_mana_cost_symbol,
  [sym_hybrid_mana_cost_symbol] = sym_hybrid_mana_cost_symbol,
  [sym_phyrexian_mana_cost_symbol] = sym_phyrexian_mana_cost_symbol,
  [sym_mana_cost_symbol] = sym_mana_cost_symbol,
  [sym_mana_cost] = sym_mana_cost,
  [aux_sym_mana_cost_repeat1] = aux_sym_mana_cost_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_Add] = {
    .visible = true,
    .named = false,
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
  [anon_sym_add] = {
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
  [anon_sym_Untapped] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_untapped] = {
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
  [anon_sym_LBRACET_RBRACE] = {
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
  [anon_sym_snow] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_world] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_land] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_artifact] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_enchantment] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_instant] = {
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
  [sym_mana_ability] = {
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
  [aux_sym_mana_cost_repeat1] = {
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
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(499);
      if (lookahead == '+') ADVANCE(627);
      if (lookahead == ',') ADVANCE(501);
      if (lookahead == '-') ADVANCE(628);
      if (lookahead == '.') ADVANCE(502);
      if (lookahead == '/') ADVANCE(588);
      if (lookahead == '2') ADVANCE(587);
      if (lookahead == ':') ADVANCE(517);
      if (lookahead == 'A') ADVANCE(510);
      if (lookahead == 'B') ADVANCE(578);
      if (lookahead == 'C') ADVANCE(586);
      if (lookahead == 'D') ADVANCE(91);
      if (lookahead == 'E') ADVANCE(474);
      if (lookahead == 'F') ADVANCE(205);
      if (lookahead == 'G') ADVANCE(582);
      if (lookahead == 'H') ADVANCE(16);
      if (lookahead == 'L') ADVANCE(201);
      if (lookahead == 'M') ADVANCE(202);
      if (lookahead == 'P') ADVANCE(590);
      if (lookahead == 'R') ADVANCE(580);
      if (lookahead == 'S') ADVANCE(515);
      if (lookahead == 'T') ADVANCE(11);
      if (lookahead == 'U') ADVANCE(576);
      if (lookahead == 'V') ADVANCE(203);
      if (lookahead == 'W') ADVANCE(574);
      if (lookahead == 'X') ADVANCE(584);
      if (lookahead == 'a') ADVANCE(511);
      if (lookahead == 'b') ADVANCE(21);
      if (lookahead == 'c') ADVANCE(12);
      if (lookahead == 'd') ADVANCE(171);
      if (lookahead == 'e') ADVANCE(17);
      if (lookahead == 'f') ADVANCE(204);
      if (lookahead == 'g') ADVANCE(36);
      if (lookahead == 'h') ADVANCE(13);
      if (lookahead == 'i') ADVANCE(297);
      if (lookahead == 'l') ADVANCE(20);
      if (lookahead == 'm') ADVANCE(227);
      if (lookahead == 'n') ADVANCE(208);
      if (lookahead == 'o') ADVANCE(301);
      if (lookahead == 'p') ADVANCE(253);
      if (lookahead == 'r') ADVANCE(117);
      if (lookahead == 's') ADVANCE(516);
      if (lookahead == 't') ADVANCE(18);
      if (lookahead == 'u') ADVANCE(313);
      if (lookahead == 'v') ADVANCE(234);
      if (lookahead == 'w') ADVANCE(199);
      if (lookahead == 'y') ADVANCE(323);
      if (lookahead == '{') ADVANCE(592);
      if (lookahead == '}') ADVANCE(593);
      if (lookahead == '~') ADVANCE(558);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(583);
      END_STATE();
    case 1:
      if (lookahead == ' ') ADVANCE(42);
      END_STATE();
    case 2:
      if (lookahead == ' ') ADVANCE(41);
      END_STATE();
    case 3:
      if (lookahead == ' ') ADVANCE(440);
      END_STATE();
    case 4:
      if (lookahead == ' ') ADVANCE(54);
      if (lookahead == 'r') ADVANCE(7);
      END_STATE();
    case 5:
      if (lookahead == ' ') ADVANCE(414);
      END_STATE();
    case 6:
      if (lookahead == ' ') ADVANCE(416);
      END_STATE();
    case 7:
      if (lookahead == ' ') ADVANCE(63);
      END_STATE();
    case 8:
      if (lookahead == ' ') ADVANCE(65);
      END_STATE();
    case 9:
      if (lookahead == '\'') ADVANCE(419);
      END_STATE();
    case 10:
      if (lookahead == '\'') ADVANCE(434);
      END_STATE();
    case 11:
      if (lookahead == 'a') ADVANCE(358);
      END_STATE();
    case 12:
      if (lookahead == 'a') ADVANCE(278);
      if (lookahead == 'o') ADVANCE(272);
      if (lookahead == 'r') ADVANCE(157);
      END_STATE();
    case 13:
      if (lookahead == 'a') ADVANCE(417);
      if (lookahead == 'e') ADVANCE(484);
      END_STATE();
    case 14:
      if (lookahead == 'a') ADVANCE(478);
      END_STATE();
    case 15:
      if (lookahead == 'a') ADVANCE(307);
      END_STATE();
    case 16:
      if (lookahead == 'a') ADVANCE(411);
      if (lookahead == 'e') ADVANCE(483);
      END_STATE();
    case 17:
      if (lookahead == 'a') ADVANCE(43);
      if (lookahead == 'i') ADVANCE(185);
      if (lookahead == 'l') ADVANCE(170);
      if (lookahead == 'n') ADVANCE(47);
      if (lookahead == 'v') ADVANCE(161);
      if (lookahead == 'x') ADVANCE(228);
      END_STATE();
    case 18:
      if (lookahead == 'a') ADVANCE(359);
      if (lookahead == 'e') ADVANCE(279);
      if (lookahead == 'h') ADVANCE(213);
      if (lookahead == 'w') ADVANCE(93);
      END_STATE();
    case 19:
      if (lookahead == 'a') ADVANCE(479);
      END_STATE();
    case 20:
      if (lookahead == 'a') ADVANCE(303);
      if (lookahead == 'e') ADVANCE(186);
      if (lookahead == 'i') ADVANCE(178);
      END_STATE();
    case 21:
      if (lookahead == 'a') ADVANCE(409);
      if (lookahead == 'l') ADVANCE(27);
      END_STATE();
    case 22:
      if (lookahead == 'a') ADVANCE(492);
      END_STATE();
    case 23:
      if (lookahead == 'a') ADVANCE(360);
      END_STATE();
    case 24:
      if (lookahead == 'a') ADVANCE(44);
      if (lookahead == 'u') ADVANCE(94);
      END_STATE();
    case 25:
      if (lookahead == 'a') ADVANCE(361);
      END_STATE();
    case 26:
      if (lookahead == 'a') ADVANCE(455);
      if (lookahead == 's') ADVANCE(442);
      END_STATE();
    case 27:
      if (lookahead == 'a') ADVANCE(46);
      if (lookahead == 'u') ADVANCE(95);
      END_STATE();
    case 28:
      if (lookahead == 'a') ADVANCE(256);
      END_STATE();
    case 29:
      if (lookahead == 'a') ADVANCE(423);
      END_STATE();
    case 30:
      if (lookahead == 'a') ADVANCE(314);
      END_STATE();
    case 31:
      if (lookahead == 'a') ADVANCE(372);
      END_STATE();
    case 32:
      if (lookahead == 'a') ADVANCE(383);
      END_STATE();
    case 33:
      if (lookahead == 'a') ADVANCE(308);
      END_STATE();
    case 34:
      if (lookahead == 'a') ADVANCE(56);
      END_STATE();
    case 35:
      if (lookahead == 'a') ADVANCE(384);
      END_STATE();
    case 36:
      if (lookahead == 'a') ADVANCE(212);
      if (lookahead == 'e') ADVANCE(421);
      if (lookahead == 'r') ADVANCE(159);
      END_STATE();
    case 37:
      if (lookahead == 'a') ADVANCE(309);
      END_STATE();
    case 38:
      if (lookahead == 'a') ADVANCE(452);
      END_STATE();
    case 39:
      if (lookahead == 'a') ADVANCE(316);
      END_STATE();
    case 40:
      if (lookahead == 'a') ADVANCE(468);
      if (lookahead == 's') ADVANCE(457);
      END_STATE();
    case 41:
      if (lookahead == 'b') ADVANCE(266);
      END_STATE();
    case 42:
      if (lookahead == 'b') ADVANCE(119);
      END_STATE();
    case 43:
      if (lookahead == 'c') ADVANCE(191);
      END_STATE();
    case 44:
      if (lookahead == 'c') ADVANCE(239);
      END_STATE();
    case 45:
      if (lookahead == 'c') ADVANCE(612);
      END_STATE();
    case 46:
      if (lookahead == 'c') ADVANCE(240);
      END_STATE();
    case 47:
      if (lookahead == 'c') ADVANCE(195);
      END_STATE();
    case 48:
      if (lookahead == 'c') ADVANCE(394);
      END_STATE();
    case 49:
      if (lookahead == 'c') ADVANCE(193);
      END_STATE();
    case 50:
      if (lookahead == 'c') ADVANCE(194);
      END_STATE();
    case 51:
      if (lookahead == 'c') ADVANCE(246);
      END_STATE();
    case 52:
      if (lookahead == 'c') ADVANCE(32);
      END_STATE();
    case 53:
      if (lookahead == 'c') ADVANCE(333);
      END_STATE();
    case 54:
      if (lookahead == 'c') ADVANCE(345);
      END_STATE();
    case 55:
      if (lookahead == 'c') ADVANCE(437);
      END_STATE();
    case 56:
      if (lookahead == 'c') ADVANCE(429);
      END_STATE();
    case 57:
      if (lookahead == 'c') ADVANCE(111);
      END_STATE();
    case 58:
      if (lookahead == 'c') ADVANCE(112);
      END_STATE();
    case 59:
      if (lookahead == 'c') ADVANCE(113);
      END_STATE();
    case 60:
      if (lookahead == 'c') ADVANCE(114);
      END_STATE();
    case 61:
      if (lookahead == 'c') ADVANCE(136);
      END_STATE();
    case 62:
      if (lookahead == 'c') ADVANCE(35);
      END_STATE();
    case 63:
      if (lookahead == 'c') ADVANCE(346);
      END_STATE();
    case 64:
      if (lookahead == 'c') ADVANCE(462);
      END_STATE();
    case 65:
      if (lookahead == 'c') ADVANCE(347);
      END_STATE();
    case 66:
      if (lookahead == 'c') ADVANCE(401);
      END_STATE();
    case 67:
      if (lookahead == 'c') ADVANCE(353);
      END_STATE();
    case 68:
      if (lookahead == 'c') ADVANCE(355);
      END_STATE();
    case 69:
      if (lookahead == 'c') ADVANCE(356);
      END_STATE();
    case 70:
      if (lookahead == 'd') ADVANCE(71);
      END_STATE();
    case 71:
      if (lookahead == 'd') ADVANCE(500);
      END_STATE();
    case 72:
      if (lookahead == 'd') ADVANCE(604);
      END_STATE();
    case 73:
      if (lookahead == 'd') ADVANCE(557);
      END_STATE();
    case 74:
      if (lookahead == 'd') ADVANCE(605);
      END_STATE();
    case 75:
      if (lookahead == 'd') ADVANCE(512);
      END_STATE();
    case 76:
      if (lookahead == 'd') ADVANCE(513);
      END_STATE();
    case 77:
      if (lookahead == 'd') ADVANCE(615);
      END_STATE();
    case 78:
      if (lookahead == 'd') ADVANCE(614);
      END_STATE();
    case 79:
      if (lookahead == 'd') ADVANCE(624);
      END_STATE();
    case 80:
      if (lookahead == 'd') ADVANCE(625);
      END_STATE();
    case 81:
      if (lookahead == 'd') ADVANCE(541);
      END_STATE();
    case 82:
      if (lookahead == 'd') ADVANCE(542);
      END_STATE();
    case 83:
      if (lookahead == 'd') ADVANCE(568);
      END_STATE();
    case 84:
      if (lookahead == 'd') ADVANCE(569);
      END_STATE();
    case 85:
      if (lookahead == 'd') ADVANCE(635);
      END_STATE();
    case 86:
      if (lookahead == 'd') ADVANCE(636);
      END_STATE();
    case 87:
      if (lookahead == 'd') ADVANCE(632);
      END_STATE();
    case 88:
      if (lookahead == 'd') ADVANCE(633);
      END_STATE();
    case 89:
      if (lookahead == 'd') ADVANCE(559);
      END_STATE();
    case 90:
      if (lookahead == 'd') ADVANCE(31);
      END_STATE();
    case 91:
      if (lookahead == 'e') ADVANCE(26);
      if (lookahead == 'i') ADVANCE(408);
      if (lookahead == 'r') ADVANCE(14);
      END_STATE();
    case 92:
      if (lookahead == 'e') ADVANCE(637);
      END_STATE();
    case 93:
      if (lookahead == 'e') ADVANCE(264);
      if (lookahead == 'o') ADVANCE(638);
      END_STATE();
    case 94:
      if (lookahead == 'e') ADVANCE(600);
      END_STATE();
    case 95:
      if (lookahead == 'e') ADVANCE(601);
      END_STATE();
    case 96:
      if (lookahead == 'e') ADVANCE(641);
      END_STATE();
    case 97:
      if (lookahead == 'e') ADVANCE(503);
      END_STATE();
    case 98:
      if (lookahead == 'e') ADVANCE(597);
      END_STATE();
    case 99:
      if (lookahead == 'e') ADVANCE(645);
      END_STATE();
    case 100:
      if (lookahead == 'e') ADVANCE(534);
      END_STATE();
    case 101:
      if (lookahead == 'e') ADVANCE(528);
      END_STATE();
    case 102:
      if (lookahead == 'e') ADVANCE(598);
      END_STATE();
    case 103:
      if (lookahead == 'e') ADVANCE(535);
      END_STATE();
    case 104:
      if (lookahead == 'e') ADVANCE(529);
      END_STATE();
    case 105:
      if (lookahead == 'e') ADVANCE(639);
      END_STATE();
    case 106:
      if (lookahead == 'e') ADVANCE(599);
      END_STATE();
    case 107:
      if (lookahead == 'e') ADVANCE(545);
      END_STATE();
    case 108:
      if (lookahead == 'e') ADVANCE(546);
      if (lookahead == 'u') ADVANCE(389);
      END_STATE();
    case 109:
      if (lookahead == 'e') ADVANCE(648);
      END_STATE();
    case 110:
      if (lookahead == 'e') ADVANCE(566);
      END_STATE();
    case 111:
      if (lookahead == 'e') ADVANCE(543);
      END_STATE();
    case 112:
      if (lookahead == 'e') ADVANCE(524);
      END_STATE();
    case 113:
      if (lookahead == 'e') ADVANCE(544);
      END_STATE();
    case 114:
      if (lookahead == 'e') ADVANCE(525);
      END_STATE();
    case 115:
      if (lookahead == 'e') ADVANCE(520);
      END_STATE();
    case 116:
      if (lookahead == 'e') ADVANCE(521);
      END_STATE();
    case 117:
      if (lookahead == 'e') ADVANCE(74);
      END_STATE();
    case 118:
      if (lookahead == 'e') ADVANCE(407);
      END_STATE();
    case 119:
      if (lookahead == 'e') ADVANCE(2);
      END_STATE();
    case 120:
      if (lookahead == 'e') ADVANCE(38);
      END_STATE();
    case 121:
      if (lookahead == 'e') ADVANCE(410);
      END_STATE();
    case 122:
      if (lookahead == 'e') ADVANCE(126);
      END_STATE();
    case 123:
      if (lookahead == 'e') ADVANCE(268);
      END_STATE();
    case 124:
      if (lookahead == 'e') ADVANCE(79);
      END_STATE();
    case 125:
      if (lookahead == 'e') ADVANCE(405);
      END_STATE();
    case 126:
      if (lookahead == 'e') ADVANCE(282);
      END_STATE();
    case 127:
      if (lookahead == 'e') ADVANCE(55);
      END_STATE();
    case 128:
      if (lookahead == 'e') ADVANCE(80);
      END_STATE();
    case 129:
      if (lookahead == 'e') ADVANCE(406);
      END_STATE();
    case 130:
      if (lookahead == 'e') ADVANCE(283);
      END_STATE();
    case 131:
      if (lookahead == 'e') ADVANCE(366);
      END_STATE();
    case 132:
      if (lookahead == 'e') ADVANCE(284);
      END_STATE();
    case 133:
      if (lookahead == 'e') ADVANCE(367);
      END_STATE();
    case 134:
      if (lookahead == 'e') ADVANCE(285);
      END_STATE();
    case 135:
      if (lookahead == 'e') ADVANCE(83);
      END_STATE();
    case 136:
      if (lookahead == 'e') ADVANCE(368);
      END_STATE();
    case 137:
      if (lookahead == 'e') ADVANCE(84);
      END_STATE();
    case 138:
      if (lookahead == 'e') ADVANCE(369);
      END_STATE();
    case 139:
      if (lookahead == 'e') ADVANCE(85);
      END_STATE();
    case 140:
      if (lookahead == 'e') ADVANCE(370);
      END_STATE();
    case 141:
      if (lookahead == 'e') ADVANCE(286);
      END_STATE();
    case 142:
      if (lookahead == 'e') ADVANCE(86);
      END_STATE();
    case 143:
      if (lookahead == 'e') ADVANCE(87);
      END_STATE();
    case 144:
      if (lookahead == 'e') ADVANCE(287);
      END_STATE();
    case 145:
      if (lookahead == 'e') ADVANCE(88);
      END_STATE();
    case 146:
      if (lookahead == 'e') ADVANCE(288);
      END_STATE();
    case 147:
      if (lookahead == 'e') ADVANCE(89);
      END_STATE();
    case 148:
      if (lookahead == 'e') ADVANCE(426);
      END_STATE();
    case 149:
      if (lookahead == 'e') ADVANCE(289);
      END_STATE();
    case 150:
      if (lookahead == 'e') ADVANCE(427);
      END_STATE();
    case 151:
      if (lookahead == 'e') ADVANCE(290);
      END_STATE();
    case 152:
      if (lookahead == 'e') ADVANCE(291);
      END_STATE();
    case 153:
      if (lookahead == 'e') ADVANCE(105);
      END_STATE();
    case 154:
      if (lookahead == 'e') ADVANCE(371);
      END_STATE();
    case 155:
      if (lookahead == 'e') ADVANCE(292);
      END_STATE();
    case 156:
      if (lookahead == 'e') ADVANCE(304);
      END_STATE();
    case 157:
      if (lookahead == 'e') ADVANCE(29);
      END_STATE();
    case 158:
      if (lookahead == 'e') ADVANCE(412);
      END_STATE();
    case 159:
      if (lookahead == 'e') ADVANCE(130);
      END_STATE();
    case 160:
      if (lookahead == 'e') ADVANCE(376);
      END_STATE();
    case 161:
      if (lookahead == 'e') ADVANCE(377);
      END_STATE();
    case 162:
      if (lookahead == 'e') ADVANCE(310);
      END_STATE();
    case 163:
      if (lookahead == 'e') ADVANCE(141);
      END_STATE();
    case 164:
      if (lookahead == 'e') ADVANCE(144);
      END_STATE();
    case 165:
      if (lookahead == 'e') ADVANCE(312);
      END_STATE();
    case 166:
      if (lookahead == 'e') ADVANCE(149);
      END_STATE();
    case 167:
      if (lookahead == 'e') ADVANCE(151);
      END_STATE();
    case 168:
      if (lookahead == 'e') ADVANCE(152);
      END_STATE();
    case 169:
      if (lookahead == 'e') ADVANCE(155);
      END_STATE();
    case 170:
      if (lookahead == 'e') ADVANCE(477);
      END_STATE();
    case 171:
      if (lookahead == 'e') ADVANCE(40);
      if (lookahead == 'i') ADVANCE(420);
      if (lookahead == 'r') ADVANCE(19);
      END_STATE();
    case 172:
      if (lookahead == 'e') ADVANCE(64);
      END_STATE();
    case 173:
      if (lookahead == 'f') ADVANCE(622);
      END_STATE();
    case 174:
      if (lookahead == 'f') ADVANCE(623);
      END_STATE();
    case 175:
      if (lookahead == 'f') ADVANCE(123);
      END_STATE();
    case 176:
      if (lookahead == 'f') ADVANCE(465);
      if (lookahead == 'r') ADVANCE(418);
      if (lookahead == 'v') ADVANCE(96);
      END_STATE();
    case 177:
      if (lookahead == 'f') ADVANCE(34);
      END_STATE();
    case 178:
      if (lookahead == 'f') ADVANCE(98);
      END_STATE();
    case 179:
      if (lookahead == 'f') ADVANCE(387);
      END_STATE();
    case 180:
      if (lookahead == 'f') ADVANCE(218);
      END_STATE();
    case 181:
      if (lookahead == 'f') ADVANCE(232);
      END_STATE();
    case 182:
      if (lookahead == 'g') ADVANCE(518);
      END_STATE();
    case 183:
      if (lookahead == 'g') ADVANCE(519);
      END_STATE();
    case 184:
      if (lookahead == 'g') ADVANCE(634);
      END_STATE();
    case 185:
      if (lookahead == 'g') ADVANCE(196);
      END_STATE();
    case 186:
      if (lookahead == 'g') ADVANCE(156);
      END_STATE();
    case 187:
      if (lookahead == 'g') ADVANCE(148);
      END_STATE();
    case 188:
      if (lookahead == 'g') ADVANCE(215);
      if (lookahead == 's') ADVANCE(219);
      END_STATE();
    case 189:
      if (lookahead == 'g') ADVANCE(150);
      END_STATE();
    case 190:
      if (lookahead == 'g') ADVANCE(236);
      if (lookahead == 's') ADVANCE(223);
      END_STATE();
    case 191:
      if (lookahead == 'h') ADVANCE(506);
      END_STATE();
    case 192:
      if (lookahead == 'h') ADVANCE(626);
      END_STATE();
    case 193:
      if (lookahead == 'h') ADVANCE(526);
      END_STATE();
    case 194:
      if (lookahead == 'h') ADVANCE(527);
      END_STATE();
    case 195:
      if (lookahead == 'h') ADVANCE(33);
      END_STATE();
    case 196:
      if (lookahead == 'h') ADVANCE(424);
      END_STATE();
    case 197:
      if (lookahead == 'h') ADVANCE(454);
      END_STATE();
    case 198:
      if (lookahead == 'h') ADVANCE(224);
      END_STATE();
    case 199:
      if (lookahead == 'h') ADVANCE(233);
      if (lookahead == 'i') ADVANCE(435);
      if (lookahead == 'o') ADVANCE(378);
      END_STATE();
    case 200:
      if (lookahead == 'h') ADVANCE(464);
      END_STATE();
    case 201:
      if (lookahead == 'i') ADVANCE(175);
      END_STATE();
    case 202:
      if (lookahead == 'i') ADVANCE(255);
      if (lookahead == 'o') ADVANCE(302);
      if (lookahead == 'u') ADVANCE(259);
      END_STATE();
    case 203:
      if (lookahead == 'i') ADVANCE(188);
      END_STATE();
    case 204:
      if (lookahead == 'i') ADVANCE(176);
      if (lookahead == 'l') ADVANCE(495);
      if (lookahead == 'o') ADVANCE(471);
      if (lookahead == 'r') ADVANCE(321);
      END_STATE();
    case 205:
      if (lookahead == 'i') ADVANCE(390);
      if (lookahead == 'l') ADVANCE(494);
      END_STATE();
    case 206:
      if (lookahead == 'i') ADVANCE(177);
      END_STATE();
    case 207:
      if (lookahead == 'i') ADVANCE(180);
      END_STATE();
    case 208:
      if (lookahead == 'i') ADVANCE(306);
      END_STATE();
    case 209:
      if (lookahead == 'i') ADVANCE(261);
      END_STATE();
    case 210:
      if (lookahead == 'i') ADVANCE(243);
      END_STATE();
    case 211:
      if (lookahead == 'i') ADVANCE(280);
      END_STATE();
    case 212:
      if (lookahead == 'i') ADVANCE(281);
      END_STATE();
    case 213:
      if (lookahead == 'i') ADVANCE(402);
      if (lookahead == 'r') ADVANCE(153);
      END_STATE();
    case 214:
      if (lookahead == 'i') ADVANCE(295);
      END_STATE();
    case 215:
      if (lookahead == 'i') ADVANCE(260);
      END_STATE();
    case 216:
      if (lookahead == 'i') ADVANCE(45);
      END_STATE();
    case 217:
      if (lookahead == 'i') ADVANCE(298);
      END_STATE();
    case 218:
      if (lookahead == 'i') ADVANCE(57);
      END_STATE();
    case 219:
      if (lookahead == 'i') ADVANCE(422);
      END_STATE();
    case 220:
      if (lookahead == 'i') ADVANCE(296);
      END_STATE();
    case 221:
      if (lookahead == 'i') ADVANCE(341);
      END_STATE();
    case 222:
      if (lookahead == 'i') ADVANCE(299);
      END_STATE();
    case 223:
      if (lookahead == 'i') ADVANCE(425);
      END_STATE();
    case 224:
      if (lookahead == 'i') ADVANCE(300);
      END_STATE();
    case 225:
      if (lookahead == 'i') ADVANCE(125);
      if (lookahead == 'y') ADVANCE(619);
      END_STATE();
    case 226:
      if (lookahead == 'i') ADVANCE(129);
      if (lookahead == 'y') ADVANCE(610);
      END_STATE();
    case 227:
      if (lookahead == 'i') ADVANCE(257);
      if (lookahead == 'o') ADVANCE(320);
      if (lookahead == 'u') ADVANCE(274);
      END_STATE();
    case 228:
      if (lookahead == 'i') ADVANCE(262);
      END_STATE();
    case 229:
      if (lookahead == 'i') ADVANCE(244);
      END_STATE();
    case 230:
      if (lookahead == 'i') ADVANCE(342);
      END_STATE();
    case 231:
      if (lookahead == 'i') ADVANCE(446);
      END_STATE();
    case 232:
      if (lookahead == 'i') ADVANCE(59);
      END_STATE();
    case 233:
      if (lookahead == 'i') ADVANCE(450);
      END_STATE();
    case 234:
      if (lookahead == 'i') ADVANCE(190);
      END_STATE();
    case 235:
      if (lookahead == 'i') ADVANCE(181);
      END_STATE();
    case 236:
      if (lookahead == 'i') ADVANCE(273);
      END_STATE();
    case 237:
      if (lookahead == 'i') ADVANCE(68);
      END_STATE();
    case 238:
      if (lookahead == 'i') ADVANCE(69);
      END_STATE();
    case 239:
      if (lookahead == 'k') ADVANCE(602);
      END_STATE();
    case 240:
      if (lookahead == 'k') ADVANCE(603);
      END_STATE();
    case 241:
      if (lookahead == 'k') ADVANCE(522);
      END_STATE();
    case 242:
      if (lookahead == 'k') ADVANCE(523);
      END_STATE();
    case 243:
      if (lookahead == 'k') ADVANCE(115);
      END_STATE();
    case 244:
      if (lookahead == 'k') ADVANCE(116);
      END_STATE();
    case 245:
      if (lookahead == 'k') ADVANCE(154);
      END_STATE();
    case 246:
      if (lookahead == 'k') ADVANCE(147);
      END_STATE();
    case 247:
      if (lookahead == 'l') ADVANCE(505);
      END_STATE();
    case 248:
      if (lookahead == 'l') ADVANCE(551);
      END_STATE();
    case 249:
      if (lookahead == 'l') ADVANCE(552);
      END_STATE();
    case 250:
      if (lookahead == 'l') ADVANCE(570);
      END_STATE();
    case 251:
      if (lookahead == 'l') ADVANCE(571);
      END_STATE();
    case 252:
      if (lookahead == 'l') ADVANCE(572);
      END_STATE();
    case 253:
      if (lookahead == 'l') ADVANCE(15);
      if (lookahead == 'r') ADVANCE(354);
      END_STATE();
    case 254:
      if (lookahead == 'l') ADVANCE(330);
      if (lookahead == 'u') ADVANCE(305);
      END_STATE();
    case 255:
      if (lookahead == 'l') ADVANCE(248);
      END_STATE();
    case 256:
      if (lookahead == 'l') ADVANCE(245);
      END_STATE();
    case 257:
      if (lookahead == 'l') ADVANCE(249);
      END_STATE();
    case 258:
      if (lookahead == 'l') ADVANCE(78);
      END_STATE();
    case 259:
      if (lookahead == 'l') ADVANCE(456);
      END_STATE();
    case 260:
      if (lookahead == 'l') ADVANCE(30);
      END_STATE();
    case 261:
      if (lookahead == 'l') ADVANCE(100);
      END_STATE();
    case 262:
      if (lookahead == 'l') ADVANCE(103);
      END_STATE();
    case 263:
      if (lookahead == 'l') ADVANCE(121);
      END_STATE();
    case 264:
      if (lookahead == 'l') ADVANCE(475);
      if (lookahead == 'n') ADVANCE(441);
      END_STATE();
    case 265:
      if (lookahead == 'l') ADVANCE(340);
      END_STATE();
    case 266:
      if (lookahead == 'l') ADVANCE(334);
      END_STATE();
    case 267:
      if (lookahead == 'l') ADVANCE(158);
      END_STATE();
    case 268:
      if (lookahead == 'l') ADVANCE(220);
      END_STATE();
    case 269:
      if (lookahead == 'l') ADVANCE(350);
      END_STATE();
    case 270:
      if (lookahead == 'l') ADVANCE(351);
      END_STATE();
    case 271:
      if (lookahead == 'l') ADVANCE(352);
      END_STATE();
    case 272:
      if (lookahead == 'l') ADVANCE(349);
      if (lookahead == 'u') ADVANCE(319);
      END_STATE();
    case 273:
      if (lookahead == 'l') ADVANCE(39);
      END_STATE();
    case 274:
      if (lookahead == 'l') ADVANCE(469);
      END_STATE();
    case 275:
      if (lookahead == 'm') ADVANCE(631);
      END_STATE();
    case 276:
      if (lookahead == 'm') ADVANCE(504);
      END_STATE();
    case 277:
      if (lookahead == 'm') ADVANCE(165);
      END_STATE();
    case 278:
      if (lookahead == 'n') ADVANCE(10);
      if (lookahead == 'r') ADVANCE(76);
      END_STATE();
    case 279:
      if (lookahead == 'n') ADVANCE(646);
      END_STATE();
    case 280:
      if (lookahead == 'n') ADVANCE(555);
      END_STATE();
    case 281:
      if (lookahead == 'n') ADVANCE(556);
      END_STATE();
    case 282:
      if (lookahead == 'n') ADVANCE(606);
      END_STATE();
    case 283:
      if (lookahead == 'n') ADVANCE(607);
      END_STATE();
    case 284:
      if (lookahead == 'n') ADVANCE(643);
      END_STATE();
    case 285:
      if (lookahead == 'n') ADVANCE(647);
      END_STATE();
    case 286:
      if (lookahead == 'n') ADVANCE(651);
      END_STATE();
    case 287:
      if (lookahead == 'n') ADVANCE(652);
      END_STATE();
    case 288:
      if (lookahead == 'n') ADVANCE(654);
      END_STATE();
    case 289:
      if (lookahead == 'n') ADVANCE(650);
      END_STATE();
    case 290:
      if (lookahead == 'n') ADVANCE(655);
      END_STATE();
    case 291:
      if (lookahead == 'n') ADVANCE(649);
      END_STATE();
    case 292:
      if (lookahead == 'n') ADVANCE(653);
      END_STATE();
    case 293:
      if (lookahead == 'n') ADVANCE(629);
      END_STATE();
    case 294:
      if (lookahead == 'n') ADVANCE(630);
      END_STATE();
    case 295:
      if (lookahead == 'n') ADVANCE(182);
      END_STATE();
    case 296:
      if (lookahead == 'n') ADVANCE(241);
      END_STATE();
    case 297:
      if (lookahead == 'n') ADVANCE(415);
      END_STATE();
    case 298:
      if (lookahead == 'n') ADVANCE(183);
      END_STATE();
    case 299:
      if (lookahead == 'n') ADVANCE(242);
      END_STATE();
    case 300:
      if (lookahead == 'n') ADVANCE(184);
      END_STATE();
    case 301:
      if (lookahead == 'n') ADVANCE(92);
      if (lookahead == 'p') ADVANCE(362);
      END_STATE();
    case 302:
      if (lookahead == 'n') ADVANCE(329);
      END_STATE();
    case 303:
      if (lookahead == 'n') ADVANCE(77);
      END_STATE();
    case 304:
      if (lookahead == 'n') ADVANCE(90);
      END_STATE();
    case 305:
      if (lookahead == 'n') ADVANCE(459);
      END_STATE();
    case 306:
      if (lookahead == 'n') ADVANCE(99);
      END_STATE();
    case 307:
      if (lookahead == 'n') ADVANCE(118);
      if (lookahead == 'y') ADVANCE(133);
      END_STATE();
    case 308:
      if (lookahead == 'n') ADVANCE(432);
      END_STATE();
    case 309:
      if (lookahead == 'n') ADVANCE(428);
      END_STATE();
    case 310:
      if (lookahead == 'n') ADVANCE(431);
      END_STATE();
    case 311:
      if (lookahead == 'n') ADVANCE(162);
      END_STATE();
    case 312:
      if (lookahead == 'n') ADVANCE(430);
      END_STATE();
    case 313:
      if (lookahead == 'n') ADVANCE(439);
      if (lookahead == 'p') ADVANCE(3);
      END_STATE();
    case 314:
      if (lookahead == 'n') ADVANCE(58);
      END_STATE();
    case 315:
      if (lookahead == 'n') ADVANCE(447);
      END_STATE();
    case 316:
      if (lookahead == 'n') ADVANCE(60);
      END_STATE();
    case 317:
      if (lookahead == 'n') ADVANCE(449);
      END_STATE();
    case 318:
      if (lookahead == 'n') ADVANCE(451);
      END_STATE();
    case 319:
      if (lookahead == 'n') ADVANCE(460);
      END_STATE();
    case 320:
      if (lookahead == 'n') ADVANCE(357);
      END_STATE();
    case 321:
      if (lookahead == 'o') ADVANCE(275);
      END_STATE();
    case 322:
      if (lookahead == 'o') ADVANCE(567);
      END_STATE();
    case 323:
      if (lookahead == 'o') ADVANCE(470);
      END_STATE();
    case 324:
      if (lookahead == 'o') ADVANCE(472);
      END_STATE();
    case 325:
      if (lookahead == 'o') ADVANCE(480);
      END_STATE();
    case 326:
      if (lookahead == 'o') ADVANCE(276);
      END_STATE();
    case 327:
      if (lookahead == 'o') ADVANCE(173);
      END_STATE();
    case 328:
      if (lookahead == 'o') ADVANCE(174);
      END_STATE();
    case 329:
      if (lookahead == 'o') ADVANCE(53);
      END_STATE();
    case 330:
      if (lookahead == 'o') ADVANCE(395);
      END_STATE();
    case 331:
      if (lookahead == 'o') ADVANCE(490);
      END_STATE();
    case 332:
      if (lookahead == 'o') ADVANCE(491);
      END_STATE();
    case 333:
      if (lookahead == 'o') ADVANCE(265);
      END_STATE();
    case 334:
      if (lookahead == 'o') ADVANCE(51);
      END_STATE();
    case 335:
      if (lookahead == 'o') ADVANCE(327);
      END_STATE();
    case 336:
      if (lookahead == 'o') ADVANCE(250);
      END_STATE();
    case 337:
      if (lookahead == 'o') ADVANCE(328);
      END_STATE();
    case 338:
      if (lookahead == 'o') ADVANCE(251);
      END_STATE();
    case 339:
      if (lookahead == 'o') ADVANCE(252);
      END_STATE();
    case 340:
      if (lookahead == 'o') ADVANCE(397);
      END_STATE();
    case 341:
      if (lookahead == 'o') ADVANCE(293);
      END_STATE();
    case 342:
      if (lookahead == 'o') ADVANCE(294);
      END_STATE();
    case 343:
      if (lookahead == 'o') ADVANCE(311);
      END_STATE();
    case 344:
      if (lookahead == 'o') ADVANCE(444);
      END_STATE();
    case 345:
      if (lookahead == 'o') ADVANCE(315);
      END_STATE();
    case 346:
      if (lookahead == 'o') ADVANCE(317);
      END_STATE();
    case 347:
      if (lookahead == 'o') ADVANCE(318);
      END_STATE();
    case 348:
      if (lookahead == 'o') ADVANCE(473);
      END_STATE();
    case 349:
      if (lookahead == 'o') ADVANCE(396);
      END_STATE();
    case 350:
      if (lookahead == 'o') ADVANCE(398);
      END_STATE();
    case 351:
      if (lookahead == 'o') ADVANCE(399);
      END_STATE();
    case 352:
      if (lookahead == 'o') ADVANCE(400);
      END_STATE();
    case 353:
      if (lookahead == 'o') ADVANCE(269);
      END_STATE();
    case 354:
      if (lookahead == 'o') ADVANCE(467);
      END_STATE();
    case 355:
      if (lookahead == 'o') ADVANCE(270);
      END_STATE();
    case 356:
      if (lookahead == 'o') ADVANCE(271);
      END_STATE();
    case 357:
      if (lookahead == 'o') ADVANCE(67);
      END_STATE();
    case 358:
      if (lookahead == 'p') ADVANCE(537);
      if (lookahead == 'r') ADVANCE(187);
      END_STATE();
    case 359:
      if (lookahead == 'p') ADVANCE(538);
      if (lookahead == 'r') ADVANCE(189);
      END_STATE();
    case 360:
      if (lookahead == 'p') ADVANCE(539);
      END_STATE();
    case 361:
      if (lookahead == 'p') ADVANCE(540);
      END_STATE();
    case 362:
      if (lookahead == 'p') ADVANCE(343);
      END_STATE();
    case 363:
      if (lookahead == 'p') ADVANCE(382);
      END_STATE();
    case 364:
      if (lookahead == 'p') ADVANCE(393);
      END_STATE();
    case 365:
      if (lookahead == 'r') ADVANCE(640);
      END_STATE();
    case 366:
      if (lookahead == 'r') ADVANCE(560);
      END_STATE();
    case 367:
      if (lookahead == 'r') ADVANCE(561);
      END_STATE();
    case 368:
      if (lookahead == 'r') ADVANCE(225);
      END_STATE();
    case 369:
      if (lookahead == 'r') ADVANCE(547);
      END_STATE();
    case 370:
      if (lookahead == 'r') ADVANCE(548);
      END_STATE();
    case 371:
      if (lookahead == 'r') ADVANCE(621);
      END_STATE();
    case 372:
      if (lookahead == 'r') ADVANCE(226);
      END_STATE();
    case 373:
      if (lookahead == 'r') ADVANCE(486);
      END_STATE();
    case 374:
      if (lookahead == 'r') ADVANCE(75);
      END_STATE();
    case 375:
      if (lookahead == 'r') ADVANCE(487);
      END_STATE();
    case 376:
      if (lookahead == 'r') ADVANCE(493);
      END_STATE();
    case 377:
      if (lookahead == 'r') ADVANCE(488);
      END_STATE();
    case 378:
      if (lookahead == 'r') ADVANCE(258);
      END_STATE();
    case 379:
      if (lookahead == 'r') ADVANCE(61);
      END_STATE();
    case 380:
      if (lookahead == 'r') ADVANCE(210);
      END_STATE();
    case 381:
      if (lookahead == 'r') ADVANCE(331);
      END_STATE();
    case 382:
      if (lookahead == 'r') ADVANCE(335);
      END_STATE();
    case 383:
      if (lookahead == 'r') ADVANCE(81);
      END_STATE();
    case 384:
      if (lookahead == 'r') ADVANCE(82);
      END_STATE();
    case 385:
      if (lookahead == 'r') ADVANCE(336);
      END_STATE();
    case 386:
      if (lookahead == 'r') ADVANCE(338);
      END_STATE();
    case 387:
      if (lookahead == 'r') ADVANCE(326);
      END_STATE();
    case 388:
      if (lookahead == 'r') ADVANCE(339);
      END_STATE();
    case 389:
      if (lookahead == 'r') ADVANCE(110);
      END_STATE();
    case 390:
      if (lookahead == 'r') ADVANCE(413);
      END_STATE();
    case 391:
      if (lookahead == 'r') ADVANCE(229);
      END_STATE();
    case 392:
      if (lookahead == 'r') ADVANCE(332);
      END_STATE();
    case 393:
      if (lookahead == 'r') ADVANCE(337);
      END_STATE();
    case 394:
      if (lookahead == 'r') ADVANCE(207);
      END_STATE();
    case 395:
      if (lookahead == 'r') ADVANCE(263);
      END_STATE();
    case 396:
      if (lookahead == 'r') ADVANCE(267);
      END_STATE();
    case 397:
      if (lookahead == 'r') ADVANCE(139);
      END_STATE();
    case 398:
      if (lookahead == 'r') ADVANCE(142);
      END_STATE();
    case 399:
      if (lookahead == 'r') ADVANCE(143);
      END_STATE();
    case 400:
      if (lookahead == 'r') ADVANCE(145);
      END_STATE();
    case 401:
      if (lookahead == 'r') ADVANCE(235);
      END_STATE();
    case 402:
      if (lookahead == 'r') ADVANCE(466);
      END_STATE();
    case 403:
      if (lookahead == 's') ADVANCE(608);
      END_STATE();
    case 404:
      if (lookahead == 's') ADVANCE(609);
      END_STATE();
    case 405:
      if (lookahead == 's') ADVANCE(620);
      END_STATE();
    case 406:
      if (lookahead == 's') ADVANCE(611);
      END_STATE();
    case 407:
      if (lookahead == 's') ADVANCE(481);
      END_STATE();
    case 408:
      if (lookahead == 's') ADVANCE(52);
      END_STATE();
    case 409:
      if (lookahead == 's') ADVANCE(216);
      END_STATE();
    case 410:
      if (lookahead == 's') ADVANCE(403);
      END_STATE();
    case 411:
      if (lookahead == 's') ADVANCE(443);
      END_STATE();
    case 412:
      if (lookahead == 's') ADVANCE(404);
      END_STATE();
    case 413:
      if (lookahead == 's') ADVANCE(433);
      END_STATE();
    case 414:
      if (lookahead == 's') ADVANCE(445);
      END_STATE();
    case 415:
      if (lookahead == 's') ADVANCE(463);
      END_STATE();
    case 416:
      if (lookahead == 's') ADVANCE(458);
      END_STATE();
    case 417:
      if (lookahead == 's') ADVANCE(448);
      if (lookahead == 'v') ADVANCE(97);
      END_STATE();
    case 418:
      if (lookahead == 's') ADVANCE(461);
      END_STATE();
    case 419:
      if (lookahead == 's') ADVANCE(8);
      END_STATE();
    case 420:
      if (lookahead == 's') ADVANCE(62);
      END_STATE();
    case 421:
      if (lookahead == 't') ADVANCE(532);
      END_STATE();
    case 422:
      if (lookahead == 't') ADVANCE(530);
      END_STATE();
    case 423:
      if (lookahead == 't') ADVANCE(108);
      END_STATE();
    case 424:
      if (lookahead == 't') ADVANCE(644);
      END_STATE();
    case 425:
      if (lookahead == 't') ADVANCE(531);
      END_STATE();
    case 426:
      if (lookahead == 't') ADVANCE(564);
      END_STATE();
    case 427:
      if (lookahead == 't') ADVANCE(565);
      END_STATE();
    case 428:
      if (lookahead == 't') ADVANCE(618);
      END_STATE();
    case 429:
      if (lookahead == 't') ADVANCE(616);
      END_STATE();
    case 430:
      if (lookahead == 't') ADVANCE(617);
      END_STATE();
    case 431:
      if (lookahead == 't') ADVANCE(9);
      END_STATE();
    case 432:
      if (lookahead == 't') ADVANCE(277);
      END_STATE();
    case 433:
      if (lookahead == 't') ADVANCE(5);
      END_STATE();
    case 434:
      if (lookahead == 't') ADVANCE(1);
      END_STATE();
    case 435:
      if (lookahead == 't') ADVANCE(192);
      END_STATE();
    case 436:
      if (lookahead == 't') ADVANCE(198);
      END_STATE();
    case 437:
      if (lookahead == 't') ADVANCE(221);
      END_STATE();
    case 438:
      if (lookahead == 't') ADVANCE(23);
      END_STATE();
    case 439:
      if (lookahead == 't') ADVANCE(25);
      END_STATE();
    case 440:
      if (lookahead == 't') ADVANCE(322);
      END_STATE();
    case 441:
      if (lookahead == 't') ADVANCE(489);
      END_STATE();
    case 442:
      if (lookahead == 't') ADVANCE(381);
      END_STATE();
    case 443:
      if (lookahead == 't') ADVANCE(101);
      END_STATE();
    case 444:
      if (lookahead == 't') ADVANCE(127);
      END_STATE();
    case 445:
      if (lookahead == 't') ADVANCE(380);
      END_STATE();
    case 446:
      if (lookahead == 't') ADVANCE(102);
      END_STATE();
    case 447:
      if (lookahead == 't') ADVANCE(385);
      END_STATE();
    case 448:
      if (lookahead == 't') ADVANCE(104);
      END_STATE();
    case 449:
      if (lookahead == 't') ADVANCE(386);
      END_STATE();
    case 450:
      if (lookahead == 't') ADVANCE(106);
      END_STATE();
    case 451:
      if (lookahead == 't') ADVANCE(388);
      END_STATE();
    case 452:
      if (lookahead == 't') ADVANCE(107);
      END_STATE();
    case 453:
      if (lookahead == 't') ADVANCE(206);
      END_STATE();
    case 454:
      if (lookahead == 't') ADVANCE(324);
      END_STATE();
    case 455:
      if (lookahead == 't') ADVANCE(197);
      END_STATE();
    case 456:
      if (lookahead == 't') ADVANCE(237);
      END_STATE();
    case 457:
      if (lookahead == 't') ADVANCE(392);
      END_STATE();
    case 458:
      if (lookahead == 't') ADVANCE(391);
      END_STATE();
    case 459:
      if (lookahead == 't') ADVANCE(138);
      END_STATE();
    case 460:
      if (lookahead == 't') ADVANCE(140);
      END_STATE();
    case 461:
      if (lookahead == 't') ADVANCE(6);
      END_STATE();
    case 462:
      if (lookahead == 't') ADVANCE(230);
      END_STATE();
    case 463:
      if (lookahead == 't') ADVANCE(37);
      END_STATE();
    case 464:
      if (lookahead == 't') ADVANCE(348);
      END_STATE();
    case 465:
      if (lookahead == 't') ADVANCE(163);
      END_STATE();
    case 466:
      if (lookahead == 't') ADVANCE(168);
      END_STATE();
    case 467:
      if (lookahead == 't') ADVANCE(172);
      END_STATE();
    case 468:
      if (lookahead == 't') ADVANCE(200);
      END_STATE();
    case 469:
      if (lookahead == 't') ADVANCE(238);
      END_STATE();
    case 470:
      if (lookahead == 'u') ADVANCE(4);
      END_STATE();
    case 471:
      if (lookahead == 'u') ADVANCE(365);
      END_STATE();
    case 472:
      if (lookahead == 'u') ADVANCE(49);
      END_STATE();
    case 473:
      if (lookahead == 'u') ADVANCE(50);
      END_STATE();
    case 474:
      if (lookahead == 'v') ADVANCE(160);
      if (lookahead == 'x') ADVANCE(209);
      END_STATE();
    case 475:
      if (lookahead == 'v') ADVANCE(109);
      END_STATE();
    case 476:
      if (lookahead == 'v') ADVANCE(132);
      END_STATE();
    case 477:
      if (lookahead == 'v') ADVANCE(134);
      END_STATE();
    case 478:
      if (lookahead == 'w') ADVANCE(549);
      END_STATE();
    case 479:
      if (lookahead == 'w') ADVANCE(550);
      END_STATE();
    case 480:
      if (lookahead == 'w') ADVANCE(613);
      END_STATE();
    case 481:
      if (lookahead == 'w') ADVANCE(28);
      END_STATE();
    case 482:
      if (lookahead == 'x') ADVANCE(642);
      END_STATE();
    case 483:
      if (lookahead == 'x') ADVANCE(363);
      END_STATE();
    case 484:
      if (lookahead == 'x') ADVANCE(364);
      END_STATE();
    case 485:
      if (lookahead == 'y') ADVANCE(596);
      END_STATE();
    case 486:
      if (lookahead == 'y') ADVANCE(553);
      END_STATE();
    case 487:
      if (lookahead == 'y') ADVANCE(554);
      END_STATE();
    case 488:
      if (lookahead == 'y') ADVANCE(507);
      END_STATE();
    case 489:
      if (lookahead == 'y') ADVANCE(656);
      END_STATE();
    case 490:
      if (lookahead == 'y') ADVANCE(533);
      END_STATE();
    case 491:
      if (lookahead == 'y') ADVANCE(536);
      END_STATE();
    case 492:
      if (lookahead == 'y') ADVANCE(131);
      END_STATE();
    case 493:
      if (lookahead == 'y') ADVANCE(436);
      END_STATE();
    case 494:
      if (lookahead == 'y') ADVANCE(214);
      END_STATE();
    case 495:
      if (lookahead == 'y') ADVANCE(217);
      END_STATE();
    case 496:
      if (lookahead == '}') ADVANCE(595);
      END_STATE();
    case 497:
      if (lookahead == '}') ADVANCE(594);
      END_STATE();
    case 498:
      if (eof) ADVANCE(499);
      if (lookahead == '2') ADVANCE(587);
      if (lookahead == 'A') ADVANCE(70);
      if (lookahead == 'B') ADVANCE(577);
      if (lookahead == 'C') ADVANCE(585);
      if (lookahead == 'G') ADVANCE(581);
      if (lookahead == 'P') ADVANCE(589);
      if (lookahead == 'R') ADVANCE(579);
      if (lookahead == 'S') ADVANCE(514);
      if (lookahead == 'U') ADVANCE(575);
      if (lookahead == 'W') ADVANCE(573);
      if (lookahead == 'X') ADVANCE(584);
      if (lookahead == '{') ADVANCE(591);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(498)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(583);
      END_STATE();
    case 499:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 500:
      ACCEPT_TOKEN(anon_sym_Add);
      END_STATE();
    case 501:
      ACCEPT_TOKEN(anon_sym_COMMA);
      END_STATE();
    case 502:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 503:
      ACCEPT_TOKEN(anon_sym_have);
      END_STATE();
    case 504:
      ACCEPT_TOKEN(anon_sym_hexprooffrom);
      END_STATE();
    case 505:
      ACCEPT_TOKEN(anon_sym_all);
      END_STATE();
    case 506:
      ACCEPT_TOKEN(anon_sym_each);
      END_STATE();
    case 507:
      ACCEPT_TOKEN(anon_sym_every);
      END_STATE();
    case 508:
      ACCEPT_TOKEN(anon_sym_Draws);
      END_STATE();
    case 509:
      ACCEPT_TOKEN(anon_sym_draws);
      END_STATE();
    case 510:
      ACCEPT_TOKEN(anon_sym_A);
      if (lookahead == 'd') ADVANCE(71);
      END_STATE();
    case 511:
      ACCEPT_TOKEN(anon_sym_a);
      if (lookahead == 'd') ADVANCE(73);
      if (lookahead == 'l') ADVANCE(247);
      if (lookahead == 'r') ADVANCE(453);
      END_STATE();
    case 512:
      ACCEPT_TOKEN(anon_sym_Card);
      END_STATE();
    case 513:
      ACCEPT_TOKEN(anon_sym_card);
      END_STATE();
    case 514:
      ACCEPT_TOKEN(anon_sym_S);
      END_STATE();
    case 515:
      ACCEPT_TOKEN(anon_sym_S);
      if (lookahead == 'a') ADVANCE(48);
      if (lookahead == 'c') ADVANCE(373);
      END_STATE();
    case 516:
      ACCEPT_TOKEN(anon_sym_s);
      if (lookahead == 'a') ADVANCE(66);
      if (lookahead == 'c') ADVANCE(375);
      if (lookahead == 'e') ADVANCE(476);
      if (lookahead == 'i') ADVANCE(482);
      if (lookahead == 'n') ADVANCE(325);
      if (lookahead == 'o') ADVANCE(379);
      END_STATE();
    case 517:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 518:
      ACCEPT_TOKEN(anon_sym_Flying);
      END_STATE();
    case 519:
      ACCEPT_TOKEN(anon_sym_flying);
      END_STATE();
    case 520:
      ACCEPT_TOKEN(anon_sym_Firststrike);
      END_STATE();
    case 521:
      ACCEPT_TOKEN(anon_sym_firststrike);
      END_STATE();
    case 522:
      ACCEPT_TOKEN(anon_sym_Lifelink);
      END_STATE();
    case 523:
      ACCEPT_TOKEN(anon_sym_lifelink);
      END_STATE();
    case 524:
      ACCEPT_TOKEN(anon_sym_Vigilance);
      END_STATE();
    case 525:
      ACCEPT_TOKEN(anon_sym_vigilance);
      END_STATE();
    case 526:
      ACCEPT_TOKEN(anon_sym_Deathtouch);
      END_STATE();
    case 527:
      ACCEPT_TOKEN(anon_sym_deathtouch);
      END_STATE();
    case 528:
      ACCEPT_TOKEN(anon_sym_Haste);
      END_STATE();
    case 529:
      ACCEPT_TOKEN(anon_sym_haste);
      END_STATE();
    case 530:
      ACCEPT_TOKEN(anon_sym_Visit);
      END_STATE();
    case 531:
      ACCEPT_TOKEN(anon_sym_visit);
      END_STATE();
    case 532:
      ACCEPT_TOKEN(anon_sym_get);
      END_STATE();
    case 533:
      ACCEPT_TOKEN(anon_sym_Destroy);
      END_STATE();
    case 534:
      ACCEPT_TOKEN(anon_sym_Exile);
      END_STATE();
    case 535:
      ACCEPT_TOKEN(anon_sym_exile);
      END_STATE();
    case 536:
      ACCEPT_TOKEN(anon_sym_destroy);
      END_STATE();
    case 537:
      ACCEPT_TOKEN(anon_sym_Tap);
      if (lookahead == 'p') ADVANCE(124);
      END_STATE();
    case 538:
      ACCEPT_TOKEN(anon_sym_tap);
      if (lookahead == 'p') ADVANCE(128);
      END_STATE();
    case 539:
      ACCEPT_TOKEN(anon_sym_Untap);
      if (lookahead == 'p') ADVANCE(135);
      END_STATE();
    case 540:
      ACCEPT_TOKEN(anon_sym_untap);
      if (lookahead == 'p') ADVANCE(137);
      END_STATE();
    case 541:
      ACCEPT_TOKEN(anon_sym_Discard);
      END_STATE();
    case 542:
      ACCEPT_TOKEN(anon_sym_discard);
      END_STATE();
    case 543:
      ACCEPT_TOKEN(anon_sym_Sacrifice);
      END_STATE();
    case 544:
      ACCEPT_TOKEN(anon_sym_sacrifice);
      END_STATE();
    case 545:
      ACCEPT_TOKEN(anon_sym_Create);
      END_STATE();
    case 546:
      ACCEPT_TOKEN(anon_sym_create);
      END_STATE();
    case 547:
      ACCEPT_TOKEN(anon_sym_Counter);
      END_STATE();
    case 548:
      ACCEPT_TOKEN(anon_sym_counter);
      END_STATE();
    case 549:
      ACCEPT_TOKEN(anon_sym_Draw);
      if (lookahead == 's') ADVANCE(508);
      END_STATE();
    case 550:
      ACCEPT_TOKEN(anon_sym_draw);
      if (lookahead == 's') ADVANCE(509);
      END_STATE();
    case 551:
      ACCEPT_TOKEN(anon_sym_Mill);
      END_STATE();
    case 552:
      ACCEPT_TOKEN(anon_sym_mill);
      END_STATE();
    case 553:
      ACCEPT_TOKEN(anon_sym_Scry);
      END_STATE();
    case 554:
      ACCEPT_TOKEN(anon_sym_scry);
      END_STATE();
    case 555:
      ACCEPT_TOKEN(anon_sym_Gain);
      END_STATE();
    case 556:
      ACCEPT_TOKEN(anon_sym_gain);
      END_STATE();
    case 557:
      ACCEPT_TOKEN(anon_sym_add);
      END_STATE();
    case 558:
      ACCEPT_TOKEN(anon_sym_TILDE);
      END_STATE();
    case 559:
      ACCEPT_TOKEN(anon_sym_can_SQUOTEtbeblocked);
      END_STATE();
    case 560:
      ACCEPT_TOKEN(anon_sym_Player);
      if (lookahead == 's') ADVANCE(562);
      END_STATE();
    case 561:
      ACCEPT_TOKEN(anon_sym_player);
      if (lookahead == 's') ADVANCE(563);
      END_STATE();
    case 562:
      ACCEPT_TOKEN(anon_sym_Players);
      END_STATE();
    case 563:
      ACCEPT_TOKEN(anon_sym_players);
      END_STATE();
    case 564:
      ACCEPT_TOKEN(anon_sym_Target);
      END_STATE();
    case 565:
      ACCEPT_TOKEN(anon_sym_target);
      END_STATE();
    case 566:
      ACCEPT_TOKEN(anon_sym_creature);
      END_STATE();
    case 567:
      ACCEPT_TOKEN(anon_sym_upto);
      END_STATE();
    case 568:
      ACCEPT_TOKEN(anon_sym_Untapped);
      END_STATE();
    case 569:
      ACCEPT_TOKEN(anon_sym_untapped);
      END_STATE();
    case 570:
      ACCEPT_TOKEN(anon_sym_youcontrol);
      END_STATE();
    case 571:
      ACCEPT_TOKEN(anon_sym_yourcontrol);
      END_STATE();
    case 572:
      ACCEPT_TOKEN(anon_sym_opponent_SQUOTEscontrol);
      END_STATE();
    case 573:
      ACCEPT_TOKEN(anon_sym_W);
      END_STATE();
    case 574:
      ACCEPT_TOKEN(anon_sym_W);
      if (lookahead == 'h') ADVANCE(231);
      END_STATE();
    case 575:
      ACCEPT_TOKEN(anon_sym_U);
      END_STATE();
    case 576:
      ACCEPT_TOKEN(anon_sym_U);
      if (lookahead == 'n') ADVANCE(438);
      END_STATE();
    case 577:
      ACCEPT_TOKEN(anon_sym_B);
      END_STATE();
    case 578:
      ACCEPT_TOKEN(anon_sym_B);
      if (lookahead == 'l') ADVANCE(24);
      END_STATE();
    case 579:
      ACCEPT_TOKEN(anon_sym_R);
      END_STATE();
    case 580:
      ACCEPT_TOKEN(anon_sym_R);
      if (lookahead == 'e') ADVANCE(72);
      END_STATE();
    case 581:
      ACCEPT_TOKEN(anon_sym_G);
      END_STATE();
    case 582:
      ACCEPT_TOKEN(anon_sym_G);
      if (lookahead == 'a') ADVANCE(211);
      if (lookahead == 'r') ADVANCE(122);
      END_STATE();
    case 583:
      ACCEPT_TOKEN(aux_sym_generic_mana_cost_symbol_token1);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(583);
      END_STATE();
    case 584:
      ACCEPT_TOKEN(anon_sym_X);
      END_STATE();
    case 585:
      ACCEPT_TOKEN(sym_colorless_mana_cost_symbol);
      END_STATE();
    case 586:
      ACCEPT_TOKEN(sym_colorless_mana_cost_symbol);
      if (lookahead == 'a') ADVANCE(374);
      if (lookahead == 'o') ADVANCE(254);
      if (lookahead == 'r') ADVANCE(120);
      END_STATE();
    case 587:
      ACCEPT_TOKEN(anon_sym_2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(583);
      END_STATE();
    case 588:
      ACCEPT_TOKEN(anon_sym_SLASH);
      END_STATE();
    case 589:
      ACCEPT_TOKEN(anon_sym_P);
      END_STATE();
    case 590:
      ACCEPT_TOKEN(anon_sym_P);
      if (lookahead == 'a') ADVANCE(485);
      if (lookahead == 'l') ADVANCE(22);
      if (lookahead == 'r') ADVANCE(344);
      END_STATE();
    case 591:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 592:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      if (lookahead == 'Q') ADVANCE(496);
      if (lookahead == 'T') ADVANCE(497);
      END_STATE();
    case 593:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 594:
      ACCEPT_TOKEN(anon_sym_LBRACET_RBRACE);
      END_STATE();
    case 595:
      ACCEPT_TOKEN(anon_sym_LBRACEQ_RBRACE);
      END_STATE();
    case 596:
      ACCEPT_TOKEN(anon_sym_Pay);
      END_STATE();
    case 597:
      ACCEPT_TOKEN(anon_sym_life);
      if (lookahead == 'l') ADVANCE(222);
      END_STATE();
    case 598:
      ACCEPT_TOKEN(anon_sym_White);
      END_STATE();
    case 599:
      ACCEPT_TOKEN(anon_sym_white);
      END_STATE();
    case 600:
      ACCEPT_TOKEN(anon_sym_Blue);
      END_STATE();
    case 601:
      ACCEPT_TOKEN(anon_sym_blue);
      END_STATE();
    case 602:
      ACCEPT_TOKEN(anon_sym_Black);
      END_STATE();
    case 603:
      ACCEPT_TOKEN(anon_sym_black);
      END_STATE();
    case 604:
      ACCEPT_TOKEN(anon_sym_Red);
      END_STATE();
    case 605:
      ACCEPT_TOKEN(anon_sym_red);
      END_STATE();
    case 606:
      ACCEPT_TOKEN(anon_sym_Green);
      END_STATE();
    case 607:
      ACCEPT_TOKEN(anon_sym_green);
      END_STATE();
    case 608:
      ACCEPT_TOKEN(anon_sym_Colorless);
      END_STATE();
    case 609:
      ACCEPT_TOKEN(anon_sym_colorless);
      END_STATE();
    case 610:
      ACCEPT_TOKEN(anon_sym_legendary);
      END_STATE();
    case 611:
      ACCEPT_TOKEN(anon_sym_legendaries);
      END_STATE();
    case 612:
      ACCEPT_TOKEN(anon_sym_basic);
      END_STATE();
    case 613:
      ACCEPT_TOKEN(anon_sym_snow);
      END_STATE();
    case 614:
      ACCEPT_TOKEN(anon_sym_world);
      END_STATE();
    case 615:
      ACCEPT_TOKEN(anon_sym_land);
      END_STATE();
    case 616:
      ACCEPT_TOKEN(anon_sym_artifact);
      END_STATE();
    case 617:
      ACCEPT_TOKEN(anon_sym_enchantment);
      END_STATE();
    case 618:
      ACCEPT_TOKEN(anon_sym_instant);
      END_STATE();
    case 619:
      ACCEPT_TOKEN(anon_sym_sorcery);
      END_STATE();
    case 620:
      ACCEPT_TOKEN(anon_sym_sorceries);
      END_STATE();
    case 621:
      ACCEPT_TOKEN(anon_sym_planeswalker);
      END_STATE();
    case 622:
      ACCEPT_TOKEN(anon_sym_Hexproof);
      END_STATE();
    case 623:
      ACCEPT_TOKEN(anon_sym_hexproof);
      if (lookahead == ' ') ADVANCE(179);
      END_STATE();
    case 624:
      ACCEPT_TOKEN(anon_sym_Tapped);
      END_STATE();
    case 625:
      ACCEPT_TOKEN(anon_sym_tapped);
      END_STATE();
    case 626:
      ACCEPT_TOKEN(anon_sym_with);
      END_STATE();
    case 627:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 628:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 629:
      ACCEPT_TOKEN(anon_sym_Protection);
      END_STATE();
    case 630:
      ACCEPT_TOKEN(anon_sym_protection);
      END_STATE();
    case 631:
      ACCEPT_TOKEN(anon_sym_from);
      END_STATE();
    case 632:
      ACCEPT_TOKEN(anon_sym_Multicolored);
      END_STATE();
    case 633:
      ACCEPT_TOKEN(anon_sym_multicolored);
      END_STATE();
    case 634:
      ACCEPT_TOKEN(anon_sym_Everything);
      END_STATE();
    case 635:
      ACCEPT_TOKEN(anon_sym_Monocolored);
      END_STATE();
    case 636:
      ACCEPT_TOKEN(anon_sym_monocolored);
      END_STATE();
    case 637:
      ACCEPT_TOKEN(anon_sym_one);
      END_STATE();
    case 638:
      ACCEPT_TOKEN(anon_sym_two);
      END_STATE();
    case 639:
      ACCEPT_TOKEN(anon_sym_three);
      END_STATE();
    case 640:
      ACCEPT_TOKEN(anon_sym_four);
      if (lookahead == 't') ADVANCE(166);
      END_STATE();
    case 641:
      ACCEPT_TOKEN(anon_sym_five);
      END_STATE();
    case 642:
      ACCEPT_TOKEN(anon_sym_six);
      if (lookahead == 't') ADVANCE(164);
      END_STATE();
    case 643:
      ACCEPT_TOKEN(anon_sym_seven);
      if (lookahead == 't') ADVANCE(169);
      END_STATE();
    case 644:
      ACCEPT_TOKEN(anon_sym_eight);
      if (lookahead == 'e') ADVANCE(146);
      END_STATE();
    case 645:
      ACCEPT_TOKEN(anon_sym_nine);
      if (lookahead == 't') ADVANCE(167);
      END_STATE();
    case 646:
      ACCEPT_TOKEN(anon_sym_ten);
      END_STATE();
    case 647:
      ACCEPT_TOKEN(anon_sym_eleven);
      END_STATE();
    case 648:
      ACCEPT_TOKEN(anon_sym_twelve);
      END_STATE();
    case 649:
      ACCEPT_TOKEN(anon_sym_thirteen);
      END_STATE();
    case 650:
      ACCEPT_TOKEN(anon_sym_fourteen);
      END_STATE();
    case 651:
      ACCEPT_TOKEN(anon_sym_fifteen);
      END_STATE();
    case 652:
      ACCEPT_TOKEN(anon_sym_sixteen);
      END_STATE();
    case 653:
      ACCEPT_TOKEN(anon_sym_seventeen);
      END_STATE();
    case 654:
      ACCEPT_TOKEN(anon_sym_eighteen);
      END_STATE();
    case 655:
      ACCEPT_TOKEN(anon_sym_nineteen);
      END_STATE();
    case 656:
      ACCEPT_TOKEN(anon_sym_twenty);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 498},
  [2] = {.lex_state = 498},
  [3] = {.lex_state = 498},
  [4] = {.lex_state = 498},
  [5] = {.lex_state = 498},
  [6] = {.lex_state = 498},
  [7] = {.lex_state = 498},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 498},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_Add] = ACTIONS(1),
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
    [anon_sym_add] = ACTIONS(1),
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
    [anon_sym_LBRACET_RBRACE] = ACTIONS(1),
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
    [anon_sym_snow] = ACTIONS(1),
    [anon_sym_world] = ACTIONS(1),
    [anon_sym_land] = ACTIONS(1),
    [anon_sym_artifact] = ACTIONS(1),
    [anon_sym_enchantment] = ACTIONS(1),
    [anon_sym_instant] = ACTIONS(1),
    [anon_sym_sorcery] = ACTIONS(1),
    [anon_sym_sorceries] = ACTIONS(1),
    [anon_sym_planeswalker] = ACTIONS(1),
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
    [sym_mana_ability] = STATE(11),
    [anon_sym_Add] = ACTIONS(3),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 9,
    ACTIONS(5), 1,
      anon_sym_S,
    ACTIONS(9), 1,
      aux_sym_generic_mana_cost_symbol_token1,
    ACTIONS(11), 1,
      anon_sym_X,
    ACTIONS(13), 1,
      sym_colorless_mana_cost_symbol,
    ACTIONS(15), 1,
      anon_sym_2,
    STATE(9), 1,
      sym_mana_color,
    STATE(17), 1,
      sym_mana_cost_symbol,
    ACTIONS(7), 5,
      anon_sym_W,
      anon_sym_U,
      anon_sym_B,
      anon_sym_R,
      anon_sym_G,
    STATE(15), 5,
      sym_plain_mana_cost_symbol,
      sym_generic_mana_cost_symbol,
      sym_snow_mana_cost_symbol,
      sym_hybrid_mana_cost_symbol,
      sym_phyrexian_mana_cost_symbol,
  [36] = 3,
    ACTIONS(17), 1,
      anon_sym_P,
    STATE(18), 1,
      sym_mana_color,
    ACTIONS(7), 5,
      anon_sym_W,
      anon_sym_U,
      anon_sym_B,
      anon_sym_R,
      anon_sym_G,
  [50] = 2,
    STATE(18), 1,
      sym_mana_color,
    ACTIONS(7), 5,
      anon_sym_W,
      anon_sym_U,
      anon_sym_B,
      anon_sym_R,
      anon_sym_G,
  [61] = 3,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    STATE(6), 1,
      aux_sym_mana_cost_repeat1,
    STATE(12), 1,
      sym_mana_cost,
  [71] = 3,
    ACTIONS(19), 1,
      anon_sym_LBRACE,
    ACTIONS(21), 1,
      ts_builtin_sym_end,
    STATE(7), 1,
      aux_sym_mana_cost_repeat1,
  [81] = 3,
    ACTIONS(23), 1,
      ts_builtin_sym_end,
    ACTIONS(25), 1,
      anon_sym_LBRACE,
    STATE(7), 1,
      aux_sym_mana_cost_repeat1,
  [91] = 1,
    ACTIONS(28), 2,
      anon_sym_SLASH,
      anon_sym_RBRACE,
  [96] = 2,
    ACTIONS(30), 1,
      anon_sym_SLASH,
    ACTIONS(32), 1,
      anon_sym_RBRACE,
  [103] = 1,
    ACTIONS(34), 2,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
  [108] = 1,
    ACTIONS(36), 1,
      ts_builtin_sym_end,
  [112] = 1,
    ACTIONS(38), 1,
      ts_builtin_sym_end,
  [116] = 1,
    ACTIONS(40), 1,
      anon_sym_RBRACE,
  [120] = 1,
    ACTIONS(42), 1,
      anon_sym_RBRACE,
  [124] = 1,
    ACTIONS(44), 1,
      anon_sym_RBRACE,
  [128] = 1,
    ACTIONS(46), 1,
      anon_sym_SLASH,
  [132] = 1,
    ACTIONS(48), 1,
      anon_sym_RBRACE,
  [136] = 1,
    ACTIONS(50), 1,
      anon_sym_RBRACE,
  [140] = 1,
    ACTIONS(52), 1,
      anon_sym_RBRACE,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 36,
  [SMALL_STATE(4)] = 50,
  [SMALL_STATE(5)] = 61,
  [SMALL_STATE(6)] = 71,
  [SMALL_STATE(7)] = 81,
  [SMALL_STATE(8)] = 91,
  [SMALL_STATE(9)] = 96,
  [SMALL_STATE(10)] = 103,
  [SMALL_STATE(11)] = 108,
  [SMALL_STATE(12)] = 112,
  [SMALL_STATE(13)] = 116,
  [SMALL_STATE(14)] = 120,
  [SMALL_STATE(15)] = 124,
  [SMALL_STATE(16)] = 128,
  [SMALL_STATE(17)] = 132,
  [SMALL_STATE(18)] = 136,
  [SMALL_STATE(19)] = 140,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(14),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_cost, 1),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_mana_cost_repeat1, 2),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_mana_cost_repeat1, 2), SHIFT_REPEAT(2),
  [28] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_color, 1),
  [30] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_plain_mana_cost_symbol, 1),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_mana_cost_repeat1, 3),
  [36] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_ability, 2),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_snow_mana_cost_symbol, 1),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_generic_mana_cost_symbol, 1),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_mana_cost_symbol, 1),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_hybrid_mana_cost_symbol, 3),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_phyrexian_mana_cost_symbol, 3),
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
