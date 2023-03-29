
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


///
/// # 実装されたルール(Wikipediaより)
/// * `カットオフ / ロールオーバー` - 手の広げられた指が５本を超える場合、（「ロールオーバー」とは対照的）その手は死ぬ。
/// ロールオーバーは余り
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
pub struct Rule {
    // 基礎A
    pub players_num: usize,
    pub default_hands: Vec<BothHands>,
    pub max_hand_value: HandValueType,
    
    // 基礎B
    pub is_divisible: bool,
    pub is_transfer_only: bool,
    pub is_division_only: bool,

    // 発展A
    pub rollover: Rollover,
    pub suicide: Suicide,
    pub reverse: bool,

    // 発展B
    pub zombie: bool,
    pub can_be_negative: bool,
}
impl Rule {
    pub fn new() {
        Self {
            players_num: DEFAULT_PLAYERS_NUM,
            default_hands: 
        }
    }
}
