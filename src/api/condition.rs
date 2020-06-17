macro_rules! enum_map {
    ($name: ident, $type:ident, $($attr: ident: $value: expr),*) => {
        pub enum $name {
           $(
            $attr,
           )*
        }

        impl $name {
            pub fn from(raw_value: $type) -> Option<Self> {
                match raw_value {
                    $(
                        $value => Some($name::$attr),
                    )*
                    _ => None
                }
            }
        }
    };
}

enum_map!(Thunderstorm, u16,
ThunderstormWithLightRain:200,
ThunderstormWithRain:201,
ThunderstormWithHeavyRain:202,
LightThunderstorm:210,
Thunderstorm:211,
HeavyThunderstorm:212,
RaggedThunderstorm:221,
ThunderstormWithLightDrizzle:230,
ThunderstormWithDrizzle:231,
ThunderstormWithHeavyDrizzle:232
);

enum_map!(Drizzle, u16, 
LightIntensityDrizzle:300,
Drizzle:301,
HeavyIntensityDrizzle:302,
LightIntensityDrizzleRain:310,
DrizzleRain:311,
HeavyIntensityDrizzleRain:312,
ShowerRainAndDrizzle:313,
HeavyShowerRainAndDrizzle:314,
ShowerDrizzle:321
);

enum_map!(Rain, u16,
LightRain:500,
ModerateRain:501,
HeavyIntensityRain:502,
VeryHeavyRain:503,
ExtremeRain:504,
FreezingRain:511,
LightIntensityShowerRain:520,
ShowerRain:521,
HeavyIntensityShowerRain:522,
RaggedShowerRain:531
);

enum_map!(Snow, u16, 
LightSnow:600,
Snow:601,
HeavySnow:602,
Sleet:611,
LightShowerSleet:612,
ShowerSleet:613,
LightRainAndSnow:615,
RainAndSnow:616,
LightShowerSnow:620,
ShowerSnow:621,
HeavyShowerSnow:622
);

enum_map!(Atmosphere, u16,
Mist:701,
Smoke:711,
Haze:721,
SandWhirls:731,
Fog:741,
Sand:751,
Dust:761,
VolcanicAsh:762,
Squalls:771,
Tornado:781
);

enum_map!(Cloud, u16, 
Clear:800,
FewClouds:801,
ScatteredClouds:802,
BrokenClouds:803,
OvercastClouds:804
);

pub enum Condition {
    Thunderstorm(Thunderstorm),
    Drizzle(Drizzle),
    Rain(Rain),
    Snow(Snow),
    Atmosphere(Atmosphere),
    Cloud(Cloud),
}

impl Condition {
    pub fn from(raw_value: u16) -> Option<Self> {
        Some(match raw_value / 100 {
            2 => Condition::Thunderstorm(Thunderstorm::from(raw_value)?),
            3 => Condition::Drizzle(Drizzle::from(raw_value)?),
            5 => Condition::Rain(Rain::from(raw_value)?),
            6 => Condition::Snow(Snow::from(raw_value)?),
            7 => Condition::Atmosphere(Atmosphere::from(raw_value)?),
            8 => Condition::Cloud(Cloud::from(raw_value)?),
            _ => return None,
        })
    }
}
