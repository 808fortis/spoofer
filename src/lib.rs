mod hook;
mod trampoline;

use std::ffi::CStr;
use libc::c_void;

struct SpoofCfg([&'static str; 8]);

#[derive(Clone, Copy)]
struct PkgEnt {
    pkg: &'static str,
    dev: u16,
}

static DEV_TABLE: &[SpoofCfg] = &[
    SpoofCfg(["nubia", "REDMAGIC 9 Pro", "ZTE", "NX769J",
              "nubia/NX769J/NX769J:14/UKQ1.230917.001/20240813.173312:user/release-keys",
              "NX769J",
              "nubia/NX769J/NX769J:14/UKQ1.230917.001/20240813.173312:user/release-keys",
              "NX769J"]),
    SpoofCfg(["Black Shark", "Black Shark 4 (China)", "Xiaomi", "2SM-X706B",
              "BlackShark/PRS-H0/Black Shark 4:13/TQ3A.230805.001/20230315:user/release-keys",
              "2SM-X706B",
              "BlackShark/PRS-H0/Black Shark 4:13/TQ3A.230805.001/20230315:user/release-keys",
              "2SM-X706B"]),
    SpoofCfg(["Xiaomi", "Xiaomi 11T Pro", "Xiaomi", "2107113SG",
              "Xiaomi/2107113SI/Mi 11T Pro:13/RKQ1.211001.001/20230410:user/release-keys",
              "2107113SG",
              "Xiaomi/2107113SI/Mi 11T Pro:13/RKQ1.211001.001/20230410:user/release-keys",
              "2107113SG"]),
    SpoofCfg(["Xiaomi", "Xiaomi 13 Pro", "Xiaomi", "2210132G",
              "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys",
              "2210132G",
              "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys",
              "2210132G"]),
    SpoofCfg(["OnePlus", "OnePlus 8 Pro 5G", "OnePlus", "IN2023",
              "OnePlus/IN2023/OnePlus8Pro:13/RKQ1.211119.001/20230501:user/release-keys",
              "IN2023",
              "OnePlus/IN2023/OnePlus8Pro:13/RKQ1.211119.001/20230501:user/release-keys",
              "IN2023"]),
    SpoofCfg(["ASUS", "ROG Phone 6D Ultimate", "ASUS", "AI2203",
              "ASUS/AI2203/ROG Phone 6D:14/UP1A.231005.007/20240315:user/release-keys",
              "AI2203",
              "ASUS/AI2203/ROG Phone 6D:14/UP1A.231005.007/20240315:user/release-keys",
              "AI2203"]),
    SpoofCfg(["Lenovo", "Legion Y700 (2023)", "Lenovo", "TB-9707F",
              "Lenovo/TB-9707F/Lenovo TB-9707F:13/TQ3A.230805.001/20230901:user/release-keys",
              "TB-9707F",
              "Lenovo/TB-9707F/Lenovo TB-9707F:13/TQ3A.230805.001/20230901:user/release-keys",
              "TB-9707F"]),
    SpoofCfg(["Xiaomi", "Xiaomi 13", "Xiaomi", "2211133G",
              "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys",
              "2211133G",
              "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys",
              "2211133G"]),
    SpoofCfg(["OnePlus", "OnePlus 13", "OnePlus", "PJZ110",
              "OnePlus/PJZ110/OP5D0DL1:15/AP3A.240617.008/V.1bd19a1-1-2:user/release-keys",
              "PJZ110",
              "OnePlus/PJZ110/OP5D0DL1:15/AP3A.240617.008/V.1bd19a1-1-2:user/release-keys",
              "PJZ110"]),
    SpoofCfg(["realme", "Realme P3 5G", "realme", "RMX5070",
              "realme/RMX5070/RMX5070:15/SKQ1.230119.001/eng.user.20250415.155201:user/release-keys",
              "RMX5070",
              "realme/RMX5070/RMX5070:15/SKQ1.230119.001/eng.user.20250415.155201:user/release-keys",
              "RMX5070"]),
    SpoofCfg(["nubia", "RedMagic 10 Pro", "ZTE", "NX789J",
              "nubia/NX789J-UN/NX789J:15/AQ3A.240812.002/20241212.194919:user/release-keys",
              "NX789J",
              "nubia/NX789J-UN/NX789J:15/AQ3A.240812.002/20241212.194919:user/release-keys",
              "NX789J"]),
    SpoofCfg(["realme", "Realme 15 Pro 5G", "realme", "RMX5101",
              "realme/RMX5101IN/RE60B4L1:15/AP3A.240617.008/V.R4T2.26cec0e-80bb4e-80b757:user/release-keys",
              "RMX5101",
              "realme/RMX5101IN/RE60B4L1:15/AP3A.240617.008/V.R4T2.26cec0e-80bb4e-80b757:user/release-keys",
              "RMX5101"]),
    SpoofCfg(["samsung", "Galaxy Z Fold 5", "samsung", "SM-F9460",
              "samsung/q2qzh/q2q:15/UP1A.231005.007/F946BXXU1BWK4:user/release-keys",
              "SM-F9460",
              "samsung/q2qzh/q2q:15/UP1A.231005.007/F946BXXU1BWK4:user/release-keys",
              "SM-F9460"]),
    SpoofCfg(["HONOR", "Honor Magic V2 RSR", "HONOR", "VER-N49DP",
              "HONOR/VER-N49DP/VER:13/ENG.20240918.123456:user/release-keys",
              "VER-N49DP",
              "HONOR/VER-N49DP/VER:13/ENG.20240918.123456:user/release-keys",
              "VER-N49DP"]),
];

static PKG_TABLE: &[PkgEnt] = &[
    PkgEnt { pkg: "com.CarXTech.highWay", dev: 0 },
    PkgEnt { pkg: "com.activision.callofduty.shooter", dev: 6 },
    PkgEnt { pkg: "com.activision.callofduty.warzone", dev: 0 },
    PkgEnt { pkg: "com.blitzteam.battleprime", dev: 0 },
    PkgEnt { pkg: "com.blizzard.diablo.immortal", dev: 0 },
    PkgEnt { pkg: "com.clashoftitansandroid.india", dev: 11 },
    PkgEnt { pkg: "com.dts.freefiremax", dev: 9 },
    PkgEnt { pkg: "com.dts.freefireth", dev: 9 },
    PkgEnt { pkg: "com.ea.game.nfs14_row", dev: 0 },
    PkgEnt { pkg: "com.ea.games.r3_row", dev: 0 },
    PkgEnt { pkg: "com.ea.gp.fifamobile", dev: 12 },
    PkgEnt { pkg: "com.gamedevltd.destinywarfare", dev: 0 },
    PkgEnt { pkg: "com.gameloft.android.ANMP.GloftA8HM", dev: 0 },
    PkgEnt { pkg: "com.gameloft.android.ANMP.GloftA9HM", dev: 13 },
    PkgEnt { pkg: "com.garena.game.codm", dev: 10 },
    PkgEnt { pkg: "com.garena.game.df", dev: 0 },
    PkgEnt { pkg: "com.garena.game.kgid", dev: 11 },
    PkgEnt { pkg: "com.garena.game.kgms", dev: 11 },
    PkgEnt { pkg: "com.garena.game.kgph", dev: 11 },
    PkgEnt { pkg: "com.garena.game.kgth", dev: 11 },
    PkgEnt { pkg: "com.garena.game.kgtw", dev: 11 },
    PkgEnt { pkg: "com.garena.game.kgvn", dev: 11 },
    PkgEnt { pkg: "com.garena.game.kgvntest", dev: 11 },
    PkgEnt { pkg: "com.kurogame.wutheringwaves.global", dev: 10 },
    PkgEnt { pkg: "com.levelinfinite.aov", dev: 11 },
    PkgEnt { pkg: "com.levelinfinite.hotta.gp", dev: 2 },
    PkgEnt { pkg: "com.levelinfinite.sgameGlobal", dev: 3 },
    PkgEnt { pkg: "com.madfingergames.legends", dev: 5 },
    PkgEnt { pkg: "com.nekki.shadowfight", dev: 0 },
    PkgEnt { pkg: "com.nekki.shadowfight3", dev: 0 },
    PkgEnt { pkg: "com.nekki.shadowfightarena", dev: 0 },
    PkgEnt { pkg: "com.netease.lztgglobal", dev: 4 },
    PkgEnt { pkg: "com.netease.newspike", dev: 0 },
    PkgEnt { pkg: "com.netflix.mediaclient", dev: 3 },
    PkgEnt { pkg: "com.netflix.ninja", dev: 3 },
    PkgEnt { pkg: "com.ngame.allstar.eu", dev: 11 },
    PkgEnt { pkg: "com.pearlabyss.blackdesertm", dev: 5 },
    PkgEnt { pkg: "com.pearlabyss.blackdesertm.gl", dev: 5 },
    PkgEnt { pkg: "com.pikpok.dr2.play", dev: 0 },
    PkgEnt { pkg: "com.proxima.dfm", dev: 8 },
    PkgEnt { pkg: "com.proximabeta.mf.uamo", dev: 1 },
    PkgEnt { pkg: "com.pubg.imobile", dev: 3 },
    PkgEnt { pkg: "com.pubg.krmobile", dev: 3 },
    PkgEnt { pkg: "com.rekoo.pubgm", dev: 3 },
    PkgEnt { pkg: "com.riotgames.league.wildrift", dev: 4 },
    PkgEnt { pkg: "com.riotgames.league.wildrifttw", dev: 4 },
    PkgEnt { pkg: "com.riotgames.league.wildriftvn", dev: 4 },
    PkgEnt { pkg: "com.ss.android.ugc.trill", dev: 0 },
    PkgEnt { pkg: "com.supercell.brawlstars", dev: 0 },
    PkgEnt { pkg: "com.supercell.clashofclans", dev: 2 },
    PkgEnt { pkg: "com.supercell.squad", dev: 0 },
    PkgEnt { pkg: "com.tencent.aovindia", dev: 11 },
    PkgEnt { pkg: "com.tencent.aovjp", dev: 11 },
    PkgEnt { pkg: "com.tencent.ig", dev: 3 },
    PkgEnt { pkg: "com.tencent.lolm", dev: 7 },
    PkgEnt { pkg: "com.tencent.ngame.chty", dev: 11 },
    PkgEnt { pkg: "com.tencent.ngjp", dev: 11 },
    PkgEnt { pkg: "com.tencent.tmgp.gnyx", dev: 0 },
    PkgEnt { pkg: "com.tencent.tmgp.kr.codm", dev: 10 },
    PkgEnt { pkg: "com.tencent.tmgp.pubgmhd", dev: 3 },
    PkgEnt { pkg: "com.tencent.tmgp.sgame", dev: 3 },
    PkgEnt { pkg: "com.vng.codmvn", dev: 10 },
    PkgEnt { pkg: "com.vng.mlbbvn", dev: 2 },
    PkgEnt { pkg: "com.vng.pubgmobile", dev: 3 },
    PkgEnt { pkg: "com.zhiliaoapp.musically", dev: 0 },
];

#[inline(always)]
fn find_device(name: &str) -> Option<&'static SpoofCfg> {
    let t = PKG_TABLE;
    let mut l = 0usize;
    let mut h = t.len();
    let name = name.as_bytes();
    while l < h {
        let m = l + (h - l) / 2;
        let e = &t[m];
        match name.cmp(e.pkg.as_bytes()) {
            std::cmp::Ordering::Less => h = m,
            std::cmp::Ordering::Greater => l = m + 1,
            std::cmp::Ordering::Equal => return Some(&DEV_TABLE[e.dev as usize]),
        }
    }
    None
}

#[repr(C)]
pub struct AppSpecializeArgs {
    pub rva: *mut c_void,
    pub pid: i32,
    pub app_data_dir: *const libc::c_char,
    pub nice_name: *const libc::c_char,
    pub process_type: *mut i64,
    pub mount_ns: *const libc::c_char,
    pub se_info: *const libc::c_char,
    pub se_name: *const libc::c_char,
    pub mount_mode: *mut i32,
    pub is_child_zygote: *mut i32,
    pub uid: *mut i64,
}

#[repr(C)]
pub struct ZygiskModule {
    pub on_module_loaded: Option<unsafe extern "C" fn(i32) -> i32>,
    pub pre_app_specialize: Option<unsafe extern "C" fn(i32, *mut c_void)>,
    pub post_app_specialize: Option<unsafe extern "C" fn(i32, *const c_void)>,
    pub on_process_loaded: Option<unsafe extern "C" fn(i32)>,
}

unsafe extern "C" fn on_module_loaded(_id: i32) -> i32 { 4 }

unsafe extern "C" fn pre_app_specialize(_id: i32, args: *mut c_void) {
    let args = args as *const AppSpecializeArgs;
    if args.is_null() { return; }
    let ptr = (*args).nice_name;
    if ptr.is_null() { return; }
    let name = match CStr::from_ptr(ptr).to_str() {
        Ok(s) => s,
        Err(_) => return,
    };
    let pkg = name.split(':').next().unwrap_or(name);
    if let Some(cfg) = find_device(pkg) {
        hook::set_spoof_props(cfg);
    }
}

unsafe extern "C" fn post_app_specialize(_id: i32, _args: *const c_void) {
    if hook::has_spoof_props() {
        hook::install_hook();
    }
}

unsafe extern "C" fn on_process_loaded(_id: i32) {}

#[no_mangle]
pub static zygisk_module: ZygiskModule = ZygiskModule {
    on_module_loaded: Some(on_module_loaded),
    pre_app_specialize: Some(pre_app_specialize),
    post_app_specialize: Some(post_app_specialize),
    on_process_loaded: Some(on_process_loaded),
};
