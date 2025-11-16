use std::collections::HashMap;
use crate::effects::{
    EffectSuite,
    Effect,
    ToggleEffect,
    SceneToggleEffect,
    LinkEffect
};

// -------------------------------------------------------------
// Helpers
// -------------------------------------------------------------

/// Map QLC min/max thresholds (0–255) to a single audio threshold in [0,1].
fn threshold(min: u8, max: u8) -> f32 {
    ((min as f32 + max as f32) / 2.0) / 255.0
}

/// Map a bar index into a (start,end) slice of the cumulative bins.
fn bar_range(bar_index: usize, total_bars: usize, n_bins: usize) -> (usize, usize) {
    if n_bins == 0 || total_bars == 0 {
        return (0, 0);
    }

    let width = n_bins as f32 / total_bars as f32;

    let start_f = bar_index as f32 * width;
    let end_f = (bar_index as f32 + 1.0) * width;

    let mut start = start_f.round() as isize;
    let mut end = end_f.round() as isize - 1;

    if start < 0 {
        start = 0;
    }
    if end < start {
        end = start;
    }
    if end as usize >= n_bins {
        end = (n_bins - 1) as isize;
    }

    (start as usize, end as usize)
}

// -------------------------------------------------------------
// 1. RB Jams  (AudioTriggers ID=10, BarsNumber=10)
// -------------------------------------------------------------
pub fn rb_jams_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 10;
    let mut suite = EffectSuite::new();

    // Bar 0 → Function 15 (Laser Red: ch2=255)
    {
        let mut chans = HashMap::new();
        chans.insert(2, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(178, 178),
        });
    }

    // Bar 1 → Function 13 (Blue: ch16=255)
    {
        let mut chans = HashMap::new();
        chans.insert(16, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // Bar 2 → Function 17 (Laser Blue: ch3=255)
    {
        let mut chans = HashMap::new();
        chans.insert(3, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(127, 153),
        });
    }

    // Bar 3 → Function 14 (UV: ch17=255)
    {
        let mut chans = HashMap::new();
        chans.insert(17, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // Bar 4 → Type=1, DMXChannels 0,5  (channel 5 level)
    suite.add(LinkEffect {
        dmx_channels: vec![5],
        bins: vec![bar_range(4, total_bars, n_bins)],
        lower: 0.0,
        upper: 1.0,
    });

    // Bar 5 → Function 8 (StrobeMode Constant: ch13=3)
    {
        let mut chans = HashMap::new();
        chans.insert(13, (3, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(5, total_bars, n_bins)],
            threshold: threshold(96, 96),
        });
    }

    // Bar 6 → Function 11 (Red: ch14=255)
    {
        let mut chans = HashMap::new();
        chans.insert(14, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(6, total_bars, n_bins)],
            threshold: threshold(51, 102),
        });
    }

    // Bar 7 → Function 16 (Laser Green: ch1=255)
    {
        let mut chans = HashMap::new();
        chans.insert(1, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(7, total_bars, n_bins)],
            threshold: threshold(102, 102),
        });
    }

    // Bar 8 → Function 12 (Green: ch15=255)
    {
        let mut chans = HashMap::new();
        chans.insert(15, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(8, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // Bar 9 → Function 10 (Strobe Max: ch13=255)
    {
        let mut chans = HashMap::new();
        chans.insert(13, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(9, total_bars, n_bins)],
            threshold: threshold(204, 204),
        });
    }

    suite
}

// -------------------------------------------------------------
// 2. Solid Wash  (ID=7, BarsNumber=5)
// -------------------------------------------------------------
pub fn solid_wash_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 5;
    let mut suite = EffectSuite::new();

    // #1 → Func 11 (Red)
    {
        let mut chans = HashMap::new();
        chans.insert(14, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(76, 76),
        });
    }

    // #2 → Func 13 (Blue)
    {
        let mut chans = HashMap::new();
        chans.insert(16, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(51, 51),
        });
    }

    // #3 → Func 20 (All Lasers: 1,2,3,4)
    {
        let mut chans = HashMap::new();
        chans.insert(1, (255, 0));
        chans.insert(2, (255, 0));
        chans.insert(3, (255, 0));
        chans.insert(4, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(204, 229),
        });
    }

    // #4 → Func 12 (Green)
    {
        let mut chans = HashMap::new();
        chans.insert(15, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(178, 229),
        });
    }

    // #5 → Func 14 (UV)
    {
        let mut chans = HashMap::new();
        chans.insert(17, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(4, total_bars, n_bins)],
            threshold: threshold(178, 229),
        });
    }

    suite
}

// -------------------------------------------------------------
// 3. Gemstone  (ID=12, BarsNumber=8)
// -------------------------------------------------------------
pub fn gemstone_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 8;
    let mut suite = EffectSuite::new();

    // #1 → Type1 DMX 0,5  (ch5)
    suite.add(LinkEffect {
        dmx_channels: vec![5],
        bins: vec![bar_range(0, total_bars, n_bins)],
        lower: 0.0,
        upper: 1.0,
    });

    // #2 → Func 20 (All Lasers)
    {
        let mut chans = HashMap::new();
        chans.insert(1, (255, 0));
        chans.insert(2, (255, 0));
        chans.insert(3, (255, 0));
        chans.insert(4, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(102, 102),
        });
    }

    // #3 → Func 14 (UV)
    {
        let mut chans = HashMap::new();
        chans.insert(17, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #4 → Func 10 (Strobe Max)
    {
        let mut chans = HashMap::new();
        chans.insert(13, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(204, 204),
        });
    }

    // #5 → Func 22 (PanelWhite: ch9)
    {
        let mut chans = HashMap::new();
        chans.insert(9, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(4, total_bars, n_bins)],
            threshold: threshold(102, 127),
        });
    }

    // #6 → Func 29 (BingBlue: ch12)
    {
        let mut chans = HashMap::new();
        chans.insert(12, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(5, total_bars, n_bins)],
            threshold: threshold(102, 127),
        });
    }

    // #7 → Func 27 (BingGreen: ch11)
    {
        let mut chans = HashMap::new();
        chans.insert(11, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(6, total_bars, n_bins)],
            threshold: threshold(102, 124),
        });
    }

    // #8 → Func 26 (BingRed: ch10)
    {
        let mut chans = HashMap::new();
        chans.insert(10, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(7, total_bars, n_bins)],
            threshold: threshold(102, 124),
        });
    }

    suite
}

// -------------------------------------------------------------
// 4. Winter Glitter  (ID=13, BarsNumber=16)
// -------------------------------------------------------------
pub fn winter_glitter_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 16;
    let mut suite = EffectSuite::new();

    // For this one every bar is Type=2 with 127/127 thresholds.

    // #1 → Func 29 (BingBlue: ch12)
    {
        let mut chans = HashMap::new();
        chans.insert(12, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #2 → Func 25 (PanelBlue: ch8)
    {
        let mut chans = HashMap::new();
        chans.insert(8, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #3 → Func 13 (Blue: ch16)
    {
        let mut chans = HashMap::new();
        chans.insert(16, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #4 → Func 14 (UV)
    {
        let mut chans = HashMap::new();
        chans.insert(17, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #5 → Func 24 (PanelGreen: ch7)
    {
        let mut chans = HashMap::new();
        chans.insert(7, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(4, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #6 → Func 12 (Green: ch15)
    {
        let mut chans = HashMap::new();
        chans.insert(15, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(5, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #7 → Func 27 (BingGreen: ch11)
    {
        let mut chans = HashMap::new();
        chans.insert(11, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(6, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #8 → Func 21 (Motor: ch5)
    {
        let mut chans = HashMap::new();
        chans.insert(5, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(7, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #9 → Func 17 (Laser Blue: ch3)
    {
        let mut chans = HashMap::new();
        chans.insert(3, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(8, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #10 → Func 16 (Laser Green: ch1)
    {
        let mut chans = HashMap::new();
        chans.insert(1, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(9, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #11 → Func 15 (Laser Red: ch2)
    {
        let mut chans = HashMap::new();
        chans.insert(2, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(10, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #12 → Func 18 (Laser Red 2: ch4)
    {
        let mut chans = HashMap::new();
        chans.insert(4, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(11, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #13 → Func 11 (Red: ch14)
    {
        let mut chans = HashMap::new();
        chans.insert(14, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(12, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #14 → Func 26 (BingRed: ch10)
    {
        let mut chans = HashMap::new();
        chans.insert(10, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(13, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #15 → Func 23 (PanelRed: ch6)
    {
        let mut chans = HashMap::new();
        chans.insert(6, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(14, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    // #16 → Func 22 (PanelWhite: ch9)
    {
        let mut chans = HashMap::new();
        chans.insert(9, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(15, total_bars, n_bins)],
            threshold: threshold(127, 127),
        });
    }

    suite
}

// -------------------------------------------------------------
// 5. Laser Focus  (ID=14, BarsNumber=5)
// -------------------------------------------------------------
pub fn laser_focus_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 5;
    let mut suite = EffectSuite::new();

    // #1 → Func 15 (Laser Red)
    {
        let mut chans = HashMap::new();
        chans.insert(2, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(140, 191),
        });
    }

    // #2 → Func 18 (Laser Red 2)
    {
        let mut chans = HashMap::new();
        chans.insert(4, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(89, 140),
        });
    }

    // #3 → Func 16 (Laser Green)
    {
        let mut chans = HashMap::new();
        chans.insert(1, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(63, 96),
        });
    }

    // #4 → Func 17 (Laser Blue)
    {
        let mut chans = HashMap::new();
        chans.insert(3, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(51, 76),
        });
    }

    // #5 → Type1 DMX 0,5  (motor ch5)
    suite.add(LinkEffect {
        dmx_channels: vec![5],
        bins: vec![bar_range(4, total_bars, n_bins)],
        lower: 0.0,
        upper: 1.0,
    });

    suite
}

// -------------------------------------------------------------
// 6. Motor Function  (ID=15, BarsNumber=8)
// -------------------------------------------------------------
pub fn motor_function_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 8;
    let mut suite = EffectSuite::new();

    // #1 → Func 21 (Motor ch5)
    {
        let mut chans = HashMap::new();
        chans.insert(5, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(102, 102),
        });
    }

    // #2 → Func 10 (Strobe Max ch13)
    {
        let mut chans = HashMap::new();
        chans.insert(13, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(226, 242),
        });
    }

    // #3 → Func 8 (StrobeMode Constant: ch13=3)
    {
        let mut chans = HashMap::new();
        chans.insert(13, (3, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(127, 226),
        });
    }

    // #4 → Func 14 (UV)
    {
        let mut chans = HashMap::new();
        chans.insert(17, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(102, 127),
        });
    }

    // #5 → Func 22 (PanelWhite ch9)
    {
        let mut chans = HashMap::new();
        chans.insert(9, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(4, total_bars, n_bins)],
            threshold: threshold(76, 127),
        });
    }

    // #6 → Func 12 (Green ch15)
    {
        let mut chans = HashMap::new();
        chans.insert(15, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(5, total_bars, n_bins)],
            threshold: threshold(51, 127),
        });
    }

    // #7 → Func 10 (Strobe Max) again
    {
        let mut chans = HashMap::new();
        chans.insert(13, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(6, total_bars, n_bins)],
            threshold: threshold(178, 204),
        });
    }

    // #8 → Func 27 (BingGreen ch11)
    {
        let mut chans = HashMap::new();
        chans.insert(11, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(7, total_bars, n_bins)],
            threshold: threshold(51, 76),
        });
    }

    suite
}

// -------------------------------------------------------------
// 7. True Lights  (ID=27, BarsNumber=9)
// -------------------------------------------------------------
pub fn true_lights_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 9;
    let mut suite = EffectSuite::new();

    // Bars 0–7: Type1 DMX to various channels
    let dmx_channels = [5u16, 7, 8, 9, 10, 11, 12, 6];
    for (i, ch) in dmx_channels.iter().enumerate() {
        suite.add(LinkEffect {
            dmx_channels: vec![*ch],
            bins: vec![bar_range(i, total_bars, n_bins)],
            lower: 0.0,
            upper: 1.0,
        });
    }

    // Bar 8 → Func 20 (All Lasers)
    {
        let mut chans = HashMap::new();
        chans.insert(1, (255, 0));
        chans.insert(2, (255, 0));
        chans.insert(3, (255, 0));
        chans.insert(4, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(8, total_bars, n_bins)],
            threshold: threshold(51, 204),
        });
    }

    suite
}

// -------------------------------------------------------------
// 8. Purp Groovin  (ID=20, BarsNumber=7)
// -------------------------------------------------------------
pub fn purp_groovin_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 7;
    let mut suite = EffectSuite::new();

    // #1 → Func 8 (StrobeMode Constant)
    {
        let mut chans = HashMap::new();
        chans.insert(13, (3, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(12, 12),
        });
    }

    // #2 → Func 14 (UV)
    {
        let mut chans = HashMap::new();
        chans.insert(17, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(102, 153),
        });
    }

    // #3 → Func 11 (Red)
    {
        let mut chans = HashMap::new();
        chans.insert(14, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(76, 127),
        });
    }

    // #4 → Type3 → slider widget 29 → DMX ch5 (treat as LinkEffect)
    suite.add(LinkEffect {
        dmx_channels: vec![5],
        bins: vec![bar_range(3, total_bars, n_bins)],
        lower: 0.0,
        upper: 1.0,
    });

    // #5 → Func 11 (Red) again
    {
        let mut chans = HashMap::new();
        chans.insert(14, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(4, total_bars, n_bins)],
            threshold: threshold(153, 178),
        });
    }

    // #6 → Func 17 (Laser Blue)
    {
        let mut chans = HashMap::new();
        chans.insert(3, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(5, total_bars, n_bins)],
            threshold: threshold(12, 12),
        });
    }

    // #7 → Func 18 (Laser Red 2)
    {
        let mut chans = HashMap::new();
        chans.insert(4, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(6, total_bars, n_bins)],
            threshold: threshold(12, 12),
        });
    }

    suite
}

// -------------------------------------------------------------
// 9. Angel Pallet  (ID=32, BarsNumber=5)
// -------------------------------------------------------------
pub fn angel_pallet_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 5;
    let mut suite = EffectSuite::new();

    // #1 → Type1 DMX 0,5 (ch5)
    suite.add(LinkEffect {
        dmx_channels: vec![5],
        bins: vec![bar_range(0, total_bars, n_bins)],
        lower: 0.0,
        upper: 1.0,
    });

    // Index=1 has no spectrum bar in XML (skip).

    // Index=2 → Func 27 (BingGreen ch11)
    {
        let mut chans = HashMap::new();
        chans.insert(11, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(178, 204),
        });
    }

    // Index=3 → Func 24 (PanelGreen ch7)
    {
        let mut chans = HashMap::new();
        chans.insert(7, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(178, 204),
        });
    }

    // Index=4 → Type1 DMX 0,9 (ch9)
    suite.add(LinkEffect {
        dmx_channels: vec![9],
        bins: vec![bar_range(4, total_bars, n_bins)],
        lower: 0.0,
        upper: 1.0,
    });

    suite
}

// -------------------------------------------------------------
// 10. RedRoom  (ID=35, BarsNumber=5)
// -------------------------------------------------------------
pub fn redroom_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 5;
    let mut suite = EffectSuite::new();

    // #1 → Func 15 (Laser Red)
    {
        let mut chans = HashMap::new();
        chans.insert(2, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(51, 127),
        });
    }

    // #2 → Func 11 (Red)
    {
        let mut chans = HashMap::new();
        chans.insert(14, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(1, total_bars, n_bins)],
            threshold: threshold(51, 204),
        });
    }

    // #3 → Func 26 (BingRed ch10)
    {
        let mut chans = HashMap::new();
        chans.insert(10, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(178, 204),
        });
    }

    // #4 → Func 18 (Laser Red2 ch4)
    {
        let mut chans = HashMap::new();
        chans.insert(4, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(102, 127),
        });
    }

    // #5 → Func 23 (PanelRed ch6)
    {
        let mut chans = HashMap::new();
        chans.insert(6, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(4, total_bars, n_bins)],
            threshold: threshold(178, 178),
        });
    }

    suite
}

// -------------------------------------------------------------
// 11. blue business  (ID=37, BarsNumber=6)
// -------------------------------------------------------------
pub fn blue_business_suite(n_bins: usize) -> EffectSuite {
    let total_bars = 6;
    let mut suite = EffectSuite::new();

    // #1 → Func 17 (Laser Blue)
    {
        let mut chans = HashMap::new();
        chans.insert(3, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(0, total_bars, n_bins)],
            threshold: threshold(102, 153),
        });
    }

    // #2 → Type3 → slider 38 → DMX ch5 (treat as LinkEffect)
    suite.add(LinkEffect {
        dmx_channels: vec![5],
        bins: vec![bar_range(1, total_bars, n_bins)],
        lower: 0.0,
        upper: 1.0,
    });

    // #3 → Func 13 (Blue)
    {
        let mut chans = HashMap::new();
        chans.insert(16, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(2, total_bars, n_bins)],
            threshold: threshold(127, 204),
        });
    }

    // #4 → Func 25 (PanelBlue ch8)
    {
        let mut chans = HashMap::new();
        chans.insert(8, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(3, total_bars, n_bins)],
            threshold: threshold(153, 204),
        });
    }

    // #5 → Func 29 (BingBlue ch12)
    {
        let mut chans = HashMap::new();
        chans.insert(12, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(4, total_bars, n_bins)],
            threshold: threshold(153, 204),
        });
    }

    // #6 → Func 14 (UV)
    {
        let mut chans = HashMap::new();
        chans.insert(17, (255, 0));
        suite.add(SceneToggleEffect {
            dmx_channels: chans,
            bins: vec![bar_range(5, total_bars, n_bins)],
            threshold: threshold(178, 204),
        });
    }

    suite
}
