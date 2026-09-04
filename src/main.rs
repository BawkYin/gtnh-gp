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
    pub heating_capacity: Option<u64>,
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
    pub heating_capacity: Option<u64>,
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
    pub heating_capacity: Option<u64>,
}

/// 子通道
pub trait SubChannel {
    /// 子通道名
    fn channel() -> String;
    /// 子通道值
    fn value(&self) -> u64;
}

/// 这是线圈方块
#[derive(Debug, Clone, Copy)]
pub enum CoilBlockKind {
    Cupronickel,       // 白铜
    Kanthal,           // 坎塔尔合金
    Nichrome,          // 镍铬合金
    TPVAlloy,          // 钛铂钒合金
    HssG,              // 高速钢-G
    Naquadah,          // 硅岩
    NaquadahAlloy,     // 硅岩合金
    ElectrumFlux,      // 通流琥珀金
    AwakenedDraconium, // 觉醒龙
    HssS,              // 高速钢-S
    Trinium,           // 三元金属
    Infinity,          // 无限
    Hypogen,           // 海珀珍
    Eternal,           // 永恒
}

pub trait CoilBlock {
    /// 返回线圈方块的基础炉温，单位 Kelvin
    fn base_heating_capacity(&self) -> u64;
}

impl CoilBlock for CoilBlockKind {
    fn base_heating_capacity(&self) -> u64 {
        match self {
            Self::Cupronickel => 1801,
            Self::Kanthal => 2701,
            Self::Nichrome => 3601,
            Self::TPVAlloy => 4501,
            Self::HssG => 5401,
            Self::Naquadah => 7201,
            Self::NaquadahAlloy => 8101,
            Self::ElectrumFlux => 9901,
            Self::AwakenedDraconium => 10801,
            Self::HssS => 6301,
            Self::Trinium => 9001,
            Self::Infinity => 11701,
            Self::Hypogen => 12601,
            Self::Eternal => 13501,
        }
    }
}

impl SubChannel for CoilBlockKind {
    fn channel() -> String {
        "coli".to_owned()
    }

    fn value(&self) -> u64 {
        match self {
            Self::Cupronickel => 1,
            Self::Kanthal => 2,
            Self::Nichrome => 3,
            Self::TPVAlloy => 4,
            Self::HssG => 5,
            Self::Naquadah => 6,
            Self::NaquadahAlloy => 7,
            Self::ElectrumFlux => 8,
            Self::AwakenedDraconium => 9,
            Self::HssS => 10,
            Self::Trinium => 11,
            Self::Infinity => 12,
            Self::Hypogen => 13,
            Self::Eternal => 14,
        }
    }
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
pub enum MultiKind {
    LargeChemicalReactor, // 大型化学反应釜
    // 工业高炉
    // coil: 线圈炉温
    ElectricBlastFurnace { coil: CoilBlockKind },
}

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
    /// 将电压装换成电压序数
    /// 方便多方块结构的计算
    pub fn voltage_n(self) -> u64 {
        match self {
            VoltageTier::ULV => 0,
            VoltageTier::LV => 1,
            VoltageTier::MV => 2,
            VoltageTier::HV => 3,
            VoltageTier::EV => 4,
            VoltageTier::IV => 5,
            VoltageTier::LUV => 6,
            VoltageTier::ZPM => 7,
            VoltageTier::UV => 8,
            VoltageTier::UHV => 9,
            VoltageTier::UEV => 10,
            VoltageTier::UIV => 11,
            VoltageTier::UMV => 12,
            VoltageTier::UXV => 13,
            VoltageTier::MAX => 14,
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
    /// 电压序数
    fn voltage_n(&self) -> u64 {
        match self.rated_power() {
            1..=8 => 0,
            9..=32 => 1,
            33..=128 => 2,
            129..=512 => 3,
            513..=2048 => 4,
            2049..=8192 => 5,
            8193..=32_768 => 6,
            32_769..=131_072 => 7,
            131_073..=524_288 => 8,
            524_289..=2_097_152 => 9,
            2_097_153..=8_388_608 => 10,
            8_388_609..=33_554_432 => 11,
            33_554_433..=134_217_728 => 12,
            134_217_729..=536_870_912 => 13,
            536_870_913..=2_147_483_640 => 14,
            2_147_483_641.. => 15,
            _ => 0,
        }
    }
    /// 功率系数
    /// 指的是对配方原始功率的折扣，一般小于等于100%
    /// 有可能需要根据配方来计算，如工业高炉
    fn power_efficiency(&self, recipe: &RawRecipe) -> f64;
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

impl Machine for SingleBlockMachine {
    /// 单方块机器的并行总是1
    fn max_parallels(&self) -> u64 {
        1
    }

    /// 单方块机器一般不省电
    fn power_efficiency(&self, _recipe: &RawRecipe) -> f64 {
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
        // 这是超过有损超频的次数
        let num = self.floor_log4(n);
        // 单方块默认 2/4 有损超频
        let time_base: i32 = 2;
        let power_base: i32 = 4;

        (time_base.pow(num) as f64, power_base.pow(num) as f64)
    }
}

/// 这个是能源仓类型
pub enum EnergyHatchKind {
    // 常规能源仓
    Regular,
    // 多安能源仓
    MultiAmp(u64),
    // 激光仓
    Laser(u64),
}

/// 定义了能源仓的行为
pub trait EnergyHatchBehavior {
    /// 能源仓的额定电压
    fn standard_voltage(&self) -> u64;
    /// 能源仓的额定电流
    fn standard_amperage(&self) -> u64;
    /// 能源仓的额定功率
    fn rated_power(&self) -> u64 {
        self.standard_voltage() * self.standard_amperage()
    }
}

// 定义了一个能源仓
pub struct EnergyHatch {
    pub kind: EnergyHatchKind,
    pub voltage: VoltageTier,
}

impl EnergyHatchBehavior for EnergyHatch {
    /// 返回能源仓的标准电压
    fn standard_voltage(&self) -> u64 {
        self.voltage.standard_voltage()
    }
    /// 返回能源仓的额定电流
    fn standard_amperage(&self) -> u64 {
        match self.kind {
            EnergyHatchKind::Regular => 2,
            EnergyHatchKind::MultiAmp(amp) => amp,
            EnergyHatchKind::Laser(amp) => amp,
        }
    }
}

pub struct MultiBlockMachine {
    pub kind: MultiKind,
    pub energy_hatches: HashMap<EnergyHatch, u64>,
}

impl Machine for MultiBlockMachine {
    /// 多方块机器的额定功率
    fn rated_power(&self) -> u64 {
        // 能源仓的个数
        let hatch_num = self.energy_hatches.len();
        // 如果只有一个能源仓
        if hatch_num == 1 {
            let (hatch, num) = self.energy_hatches.iter().next().unwrap();

            match hatch.kind {
                EnergyHatchKind::Regular => hatch.rated_power() * num / 2,
                EnergyHatchKind::MultiAmp(_) => hatch.rated_power() * num,
                EnergyHatchKind::Laser(_) => hatch.rated_power() * num,
            }
        }
        // 如果有多个能源仓
        else {
            self.energy_hatches
                .iter()
                .map(|h| {
                    let (hatch, num) = h;
                    hatch.rated_power() * num
                })
                .sum()
        }
    }

    fn max_parallels(&self) -> u64 {
        match self.kind {
            MultiKind::LargeChemicalReactor | MultiKind::ElectricBlastFurnace { coil: _ } => 1,
        }
    }

    fn power_efficiency(&self, recipe: &RawRecipe) -> f64 {
        match self.kind {
            // 没有功率折扣的机器
            // 大型化学反应釜
            MultiKind::LargeChemicalReactor => 1.0,
            // 工业高炉
            // 每高过配方炉温900K，就累乘 0.95
            MultiKind::ElectricBlastFurnace { coil } => {
                // 基础炉温
                let base_heating_capacity = coil.base_heating_capacity();
                // 工业高炉的电压每超过MV一个等级获得100K的加成
                let voltage_n = self.voltage_n() - VoltageTier::MV.voltage_n();
                let increases_heat = if voltage_n > 0 { 100 * voltage_n } else { 0 };
                // 获得工业高炉的炉温
                let heating_capacity = base_heating_capacity + increases_heat;
                // 获得配方需要的炉温
                let recipe_heating_capacity = recipe.heating_capacity.unwrap();
                // 计算功率折扣
                if base_heating_capacity < recipe_heating_capacity {
                    panic!("工业高炉烧不了这个配方")
                } else {
                    // 工业高炉炉温超过配方炉温900K的次数
                    let n = (heating_capacity - recipe_heating_capacity) / 900;
                    // 最终的功率折扣系数
                    let mut efficiency = 1.0;
                    for _ in 0..n {
                        efficiency *= 0.95;
                    }
                    efficiency
                }
            }
        }
    }

    fn operation_speed(&self) -> f64 {
        match self.kind {
            // 没有速度减免的机器有：
            // 工业高炉  大型化学反应釜
            MultiKind::LargeChemicalReactor | MultiKind::ElectricBlastFurnace { coil: _ } => 1.0,
        }
    }
    /// 多方块机器的电压等级
    /// 有四种类型：提升一级、无法提升、降低一级、不受限制
    fn voltage_level(&self) -> u64 {
        match self.kind {
            // 默认是提升一级
            MultiKind::LargeChemicalReactor | MultiKind::ElectricBlastFurnace { coil: _ } => {
                4 * self
                    .energy_hatches
                    .iter()
                    .map(|h| h.0.standard_voltage())
                    .sum::<u64>()
                    / self.energy_hatches.len() as u64
            }
        }
    }
    fn calculate_overclock(&self, recipe: &ActualRecipe) -> (f64, f64) {
        match self.kind {
            // 无条件无损超频的机器
            // 大型化学反应釜
            MultiKind::LargeChemicalReactor => {
                // 配方实际功率
                let mut recipe_power = recipe.power.0;
                // 机器的电压等级
                let machine_voltage_level = self.voltage_level();
                if (machine_voltage_level as f64) < recipe_power {
                    panic!("电压等级不够");
                }
                // 机器的额定功率
                let machine_rated_power = self.rated_power();
                if (machine_rated_power as f64) < recipe_power {
                    panic!("额定功率不够");
                }
                // 无损超频的次数
                let mut n = 1;
                while recipe_power < (machine_rated_power as f64) {
                    recipe_power *= 4.0;
                    n += 1;
                }
                (4_f64.powi(n), 4_f64.powi(n))
            }
            // 无损超频和有损超频混合的机器
            // 工业高炉
            MultiKind::ElectricBlastFurnace { coil } => {
                // 配方实际功率
                let mut recipe_power = recipe.power.0;
                // 机器的电压等级
                let machine_voltage_level = self.voltage_level();
                if (machine_voltage_level as f64) < recipe_power {
                    panic!("电压等级不够");
                }
                // 机器的额定功率
                let machine_rated_power = self.rated_power();
                if (machine_rated_power as f64) < recipe_power {
                    panic!("额定功率不够");
                }
                let mut n_imperfect = 1;
                while recipe_power < (machine_rated_power as f64) {
                    recipe_power *= 4.0;
                    n_imperfect += 1;
                }
                let n_perfect =
                    (coil.base_heating_capacity() - recipe.heating_capacity.unwrap()) / 1800;
                // 最后的混合倍率
                (
                    4_f64.powi(n_perfect as i32) * 2_f64.powi(n_imperfect - (n_perfect as i32)),
                    4_f64.powi(n_imperfect),
                )
            }
        }
    }
}

pub enum AnyMachine {
    SingleBlock(SingleBlockMachine),
    MultiBlock(MultiBlockMachine),
}

impl Machine for AnyMachine {
    fn max_parallels(&self) -> u64 {
        match self {
            Self::SingleBlock(machine) => machine.max_parallels(),
            Self::MultiBlock(machine) => machine.max_parallels(),
        }
    }
    fn calculate_overclock(&self, recipe: &ActualRecipe) -> (f64, f64) {
        match self {
            Self::MultiBlock(machine) => machine.calculate_overclock(recipe),
            Self::SingleBlock(machine) => machine.calculate_overclock(recipe),
        }
    }
    fn rated_power(&self) -> u64 {
        match self {
            Self::SingleBlock(machine) => machine.rated_power(),
            Self::MultiBlock(machine) => machine.rated_power(),
        }
    }
    fn voltage_level(&self) -> u64 {
        match self {
            Self::SingleBlock(machine) => machine.voltage_level(),
            Self::MultiBlock(machine) => machine.voltage_level(),
        }
    }
    fn power_efficiency(&self, recipe: &RawRecipe) -> f64 {
        match self {
            Self::MultiBlock(machine) => machine.power_efficiency(recipe),
            Self::SingleBlock(machine) => machine.power_efficiency(recipe),
        }
    }
    fn operation_speed(&self) -> f64 {
        match self {
            Self::SingleBlock(machine) => machine.operation_speed(),
            Self::MultiBlock(machine) => machine.operation_speed(),
        }
    }
}
