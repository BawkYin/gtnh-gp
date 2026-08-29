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

/// 这是单方块机器
pub enum SingleKind {
    /// 处理类机器
    Assembler, // 组装机
    CircuitAssembler, // 电路组装机
    Mixer,            //搅拌机
    LaserEngraver,    // 激光蚀刻机
    CuttingMachine,   // 切割机
    Compressor,       // 压缩机
    Autoclave,        // 高压釜
    /// 锻造类机器
    BendingMachine, // 卷板机
    FormingPress,     // 冲压机床
    ForgeHammer,      // 锻造锤
    Extruder,         // 压膜机
    Lathe,            // 车床
    Wiremill,         // 线材轧机
    /// 分离类机器
    Extractor, // 提取机
    FluidExtractor,   // 流体提取机
    Centrifuge,       // 离心机
    Electrolyzer,     // 电解机
    ElectromagneticSeparator, // 电磁离析机
    ElectromagneticPolarizer, // 两极磁化机
    Dehydrator,       // 化学脱水机
    /// 化学类机器
    ChemicalReactor, // 化学反应釜
    Distillery,       // 蒸馏室
    Brewery,          // 酿造室
    Fermenter,        // 发酵槽
    BioLab,           // 生物实验室
    ColdTrap,         // 冷阱
    ReactorProcessingUnit, // 反应堆处理单元
    /// 加热类机器
    ElectricFurnace, // 电炉
    ElectricOven,     // 电烤炉
    Microwave,        // 微波炉
    AlloySmelter,     // 合金炉
    FluidHeater,      // 流体加热器
    ArcFurnace,       // 电弧炉
    /// 矿石处理类机器
    SeismicProspector, // 地震勘探者
    Pump,             // 泵
    Miner,            // 采矿机
    Macerator,        // 研磨机
    SimpleWasher,     // 简易洗矿池
    OreWasher,        // 洗矿厂
    ChemicalBath,     // 化学浸洗机
    Sifter,           // 筛选机
    ThermalCentrifuge, // 热力离心机
    /// 打包类机器
    Canner, // 装罐机
    Packager,         // 打包机
    Unpackager,       // 解包器
    /// 其他机器
    RockBreaker, // 碎石机
    AutoChisel,       // 自动雕凿机
}

/// 这是多方块机器
/// 用到的时候在写
pub enum MultiKind {}

pub enum AnyMachine {
    SingleBlock(SingleKind),
    MultiBlock(MultiKind),
}
