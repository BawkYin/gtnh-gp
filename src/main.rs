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

// 电压等级
pub enum VoltageTier {
    ULV,
    LV,
    MV,
    HV,
    EV,
    IV,
    LUV,
    ZPM,
    UV,
    UHV,
    UEV,
    UIV,
    UMV,
    UXV,
    MAX,
}

impl VoltageTier {
    /// 将电压转换成对应的电压
    pub fn standard_voltage(&self) -> u64 {
        match self {
            VoltageTier::ULV => 8,
            VoltageTier::LV => 32,
            VoltageTier::MV => 128,
            VoltageTier::HV => 512,
            VoltageTier::EV => 2048,
            VoltageTier::IV => 8192,
            VoltageTier::LUV => 32768,
            VoltageTier::ZPM => 131072,
            VoltageTier::UV => 524288,
            VoltageTier::UHV => 2097152,
            VoltageTier::UEV => 8388608,
            VoltageTier::UIV => 33554432,
            VoltageTier::UMV => 134217728,
            VoltageTier::UXV => 536870912,
            VoltageTier::MAX => 2147483640,
        }
    }
}

pub trait Machine {
    /// 最大并行
    fn max_parallels(&self) -> u64;
    /// 额定功率
    fn rated_power(&self) -> u64;
    /// 电压等级
    fn voltage_level(&self) -> u64;
    /// 功率系数
    /// 指的是对配方原始功率的折扣，一般小于等于100%
    fn power_efficiency(&self) -> f64;
    /// 运行速度
    /// 指的是对配方原始执行时间的折扣，一般大于等于100%
    fn operation_speed(&self) -> f64;
    /// 计算超频
    /// 返回超频后的时间和功率倍率
    /// 如一次无损超频返回 (2, 4)，两次无损超频返回 (4, 16)
    /// 这样就兼容混合超频，特殊超频等
    fn calculate_overclock(&self, recipe: &ActualRecipe) -> (f64, f64);
    /// 对 4 求指数，即 floor(log_4(n))
    /// 这是计算超频的辅助函数
    // TODO: 需要测试
    fn floor_log4(&self, n: f64) -> u32 {
        let n = n;
        let eps = 1e-12;

        let num_float: f64 = n.log2() / 2.0;
        let mut num = num_float.floor() as i32;

        if 4.0_f64.powi(num) > n + eps {
            num -= 1;
        }

        while 4.0_f64.powi(num + 1) <= n + eps {
            num += 1;
        }

        num as u32
    }
}

/// 单方块机器
/// 所有属性基本固定
/// 少数属性只要电压就能计算
/// 所以有了这个结构体
pub struct SingleBlockMachine {
    pub kind: SingleKind,
    pub voltage: VoltageTier,
}

pub struct MultiBlockMachine {}

impl Machine for SingleBlockMachine {
    /// 单方块机器的并行总是1
    fn max_parallels(&self) -> u64 {
        1
    }

    /// 单方块机器一般不省电
    fn power_efficiency(&self) -> f64 {
        1.0
    }

    /// 单方块机器一般不省时间
    fn operation_speed(&self) -> f64 {
        1.0
    }

    fn rated_power(&self) -> u64 {
        let standard_voltage: u64 = self.voltage.standard_voltage();
        let standard_amperage: u64 = match self.kind {
            // 电弧炉的额定功率是 3A * 标准电压
            SingleKind::ArcFurnace => 3,
            // 热力离心机的额定功率是 2A * 标准电压
            SingleKind::ThermalCentrifuge => 2,
            // 其他机器的额定功率是 1A * 标准电压
            _ => 1,
        };
        standard_voltage * standard_amperage
    }

    /// 单方块机器的电压等级与额定功率相等
    fn voltage_level(&self) -> u64 {
        self.rated_power()
    }

    /// 单方块机器的超频基本上是有损超频
    /// 除了质量发生器，但是我没有加到枚举里面
    fn calculate_overclock(&self, recipe: &ActualRecipe) -> (f64, f64) {
        let actual_power = recipe.power.0;
        let rated_power = self.rated_power();
        // 这是额定功率和实际功率的比值
        let n = rated_power as f64 / actual_power;

        let num = self.floor_log4(n);

        let time_base: i32 = 2;
        let power_base: i32 = 4;

        (time_base.pow(num) as f64, power_base.pow(num) as f64)
    }
}

pub enum AnyMachine {
    SingleBlock(SingleKind),
    MultiBlock(MultiKind),
}
