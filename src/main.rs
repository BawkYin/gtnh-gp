use std::collections::HashMap;

fn main() {}

/// 配方的原始执行时间，单位为 tick
pub struct RawRecipeDuration(u64);
/// 配方的原始功率，单位为 EU/tick
pub struct RawRecipePower(u64);
/// 原始配方
pub struct RawRecipe {
    pub power: RawRecipePower,
    pub duration: RawRecipeDuration,
    pub inputs: HashMap<String, u64>,
    pub outputs: HashMap<String, (u64, f64)>,
    pub catalysts: HashMap<String, u64>,
}

/// 配方的实际执行速度，超频前，折扣后
pub struct ActualRecipeDuration(f64);
/// 配方的实际功率，超频前，折扣后
pub struct ActualRecipePower(f64);
/// 实际配方，超频前，折扣后
pub struct ActualRecipe {
    pub power: ActualRecipePower,
    pub duration: ActualRecipeDuration,
    pub inputs: HashMap<String, u64>,
    pub outputs: HashMap<String, (u64, f64)>,
    pub catalysts: HashMap<String, u64>,
}

/// 配方的最终执行速度，超频后，折扣后
/// 向下取整，若为0，则强制为1tick，触发1tOC
pub struct OverClockRecipeDuration(u64);
/// 配方的最终功率，超频后，折扣后
/// 向上取整
pub struct OverClockRecipePower(u64);
/// 最终的配方
pub struct OverClockRecipe {
    pub power: OverClockRecipePower,
    pub duration: OverClockRecipeDuration,
    pub inputs: HashMap<String, u64>,
    pub outputs: HashMap<String, (u64, f64)>,
    pub catalysts: HashMap<String, u64>,
}
