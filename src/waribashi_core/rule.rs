
mod rollover;
mod suicide;

use rollover::Rollover;
use std::vec::Vec;
use suicide::Suicide;

use super::{
    both_hands::BothHands,
    hand::HandValueType
};

const DEFAULT_PLAYERS_NUM: usize = 2;
const DEFAULT_MAX_HAND_VALUE: HandValueType = 5;

const DEFAULT_IS_DIVISIBLE: bool = true;
const DEFAULT_IS_TRANSFERABLE: bool = true;

const DEFAULT_ENABLE_REVERSE: bool = false;

const DEFAULT_ENABLE_ZOMBIE: bool = false;
const DEFAULT_CAN_BE_NEGATIVE: bool = false;



///
/// # 実装されたルール(Wikipediaより)
/// * `カットオフ / ロールオーバー` - 手の広げられた指が５本を超える場合、（「ロールオーバー」とは対照的）その手は死ぬ。
/// ロールオーバーは余りである。
/// * `自殺` - プレイヤーは分割で自分の手の１つを殺すことができる。
/// * `逆形` - 最初に自分の全ての手が死んだプレイヤーが勝利となる。
/// * `ゾンビ` - ３人以上のプレイヤーがいる場合、ゲームに負けたプレイヤーは、片方の手で指を１本広げた状態（すなわち01）でゲームが終わるまでプレイに残り続ける。
/// それらのプレイヤーは、自身のターンで攻撃することはできるが、分割はできない。
/// また、他のプレイヤーはそれらの手を攻撃することはできない（Chris Bandyによって発明された）。
/// * `整数` - 手の表裏を区別し、手が表(手の甲が上向き)である場合は指の本数は正の値、裏(手の甲が下向き)である場合は負の値を表す。
/// 例えば、手が裏向きで２本の指が広げられている場合、その手の値は-2であることを意味する。
/// このとき、広げられた指の本数が０であるような生きている手も可能となるが（値３の手が値-３の手に攻撃された場合など）、手は値５または値-５になったときに死ぬ。
/// また、プレイヤーは自身のターンで手を表裏ひっくり返し、手の表す値の＋/－符号(正負)を変えることが許されている。
/// 正負を変化させるこの移動は、ロールオーバーにおいて１つの手の表す値を、５からその値を引いたものに置き換える移動と等価である。
///
#[derive(Clone)]
pub struct Rule {
    // 基礎A
    pub players_num: usize,
    pub default_hands: Vec<BothHands>,
    pub max_hand_value: HandValueType,

    // 基礎B
    pub is_divisible: bool,
    pub is_transferable: bool,

    // 発展A
    pub enable_rollover: Rollover,
    pub enable_suicide: Suicide,
    pub enable_reverse: bool,

    // 発展B
    pub enable_zombie: bool,
    pub can_be_negative: bool,
}
impl Rule {
    pub fn new() -> Self {
        let new_players_num = DEFAULT_PLAYERS_NUM;

        let temp_hands = BothHands::new();
        let mut new_default_hands = Vec::new();
        new_default_hands.resize_with(new_players_num, || { temp_hands.clone() });

        Self {
            players_num: new_players_num,
            default_hands: new_default_hands,
            max_hand_value: DEFAULT_MAX_HAND_VALUE,

            is_divisible: DEFAULT_IS_DIVISIBLE,
            is_transferable: DEFAULT_IS_TRANSFERABLE,

            enable_rollover: Rollover::new(),
            enable_suicide: Suicide::new(),
            enable_reverse: DEFAULT_ENABLE_REVERSE,

            enable_zombie: DEFAULT_ENABLE_ZOMBIE,
            can_be_negative: DEFAULT_CAN_BE_NEGATIVE
        }
    }
}
