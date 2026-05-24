mod hook;
mod trampoline;

use std::ffi::CStr;
use libc::c_void;

struct PropPair {
    k: &'static str,
    v: &'static str,
}

struct SpoofCfg {
    props: &'static [PropPair],
}

#[derive(Clone, Copy)]
struct PkgEnt {
    pkg: &'static str,
    dev: u16,
}

static DEV_TABLE: &[SpoofCfg] = &[
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "nubia" },
        PropPair { k: "ro.product.device", v: "REDMAGIC 9 Pro" },
        PropPair { k: "ro.product.manufacturer", v: "ZTE" },
        PropPair { k: "ro.product.model", v: "NX769J" },
        PropPair { k: "ro.product.fingerprint", v: "nubia/NX769J/NX769J:14/UKQ1.230917.001/20240813.173312:user/release-keys" },
        PropPair { k: "ro.product.product", v: "NX769J" },
        PropPair { k: "ro.build.fingerprint", v: "nubia/NX769J/NX769J:14/UKQ1.230917.001/20240813.173312:user/release-keys" },
        PropPair { k: "ro.build.product", v: "NX769J" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "Black Shark" },
        PropPair { k: "ro.product.device", v: "Black Shark 4 (China)" },
        PropPair { k: "ro.product.manufacturer", v: "Xiaomi" },
        PropPair { k: "ro.product.model", v: "2SM-X706B" },
        PropPair { k: "ro.product.fingerprint", v: "BlackShark/PRS-H0/Black Shark 4:13/TQ3A.230805.001/20230315:user/release-keys" },
        PropPair { k: "ro.product.product", v: "2SM-X706B" },
        PropPair { k: "ro.build.fingerprint", v: "BlackShark/PRS-H0/Black Shark 4:13/TQ3A.230805.001/20230315:user/release-keys" },
        PropPair { k: "ro.build.product", v: "2SM-X706B" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "Xiaomi" },
        PropPair { k: "ro.product.device", v: "Xiaomi 11T Pro" },
        PropPair { k: "ro.product.manufacturer", v: "Xiaomi" },
        PropPair { k: "ro.product.model", v: "2107113SG" },
        PropPair { k: "ro.product.fingerprint", v: "Xiaomi/2107113SI/Mi 11T Pro:13/RKQ1.211001.001/20230410:user/release-keys" },
        PropPair { k: "ro.product.product", v: "2107113SG" },
        PropPair { k: "ro.build.fingerprint", v: "Xiaomi/2107113SI/Mi 11T Pro:13/RKQ1.211001.001/20230410:user/release-keys" },
        PropPair { k: "ro.build.product", v: "2107113SG" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "Xiaomi" },
        PropPair { k: "ro.product.device", v: "Xiaomi 13 Pro" },
        PropPair { k: "ro.product.manufacturer", v: "Xiaomi" },
        PropPair { k: "ro.product.model", v: "2210132G" },
        PropPair { k: "ro.product.fingerprint", v: "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys" },
        PropPair { k: "ro.product.product", v: "2210132G" },
        PropPair { k: "ro.build.fingerprint", v: "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys" },
        PropPair { k: "ro.build.product", v: "2210132G" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "OnePlus" },
        PropPair { k: "ro.product.device", v: "OnePlus 8 Pro 5G" },
        PropPair { k: "ro.product.manufacturer", v: "OnePlus" },
        PropPair { k: "ro.product.model", v: "IN2023" },
        PropPair { k: "ro.product.fingerprint", v: "OnePlus/IN2023/OnePlus8Pro:13/RKQ1.211119.001/20230501:user/release-keys" },
        PropPair { k: "ro.product.product", v: "IN2023" },
        PropPair { k: "ro.build.fingerprint", v: "OnePlus/IN2023/OnePlus8Pro:13/RKQ1.211119.001/20230501:user/release-keys" },
        PropPair { k: "ro.build.product", v: "IN2023" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "ASUS" },
        PropPair { k: "ro.product.device", v: "ROG Phone 6D Ultimate" },
        PropPair { k: "ro.product.manufacturer", v: "ASUS" },
        PropPair { k: "ro.product.model", v: "AI2203" },
        PropPair { k: "ro.product.fingerprint", v: "ASUS/AI2203/ROG Phone 6D:14/UP1A.231005.007/20240315:user/release-keys" },
        PropPair { k: "ro.product.product", v: "AI2203" },
        PropPair { k: "ro.build.fingerprint", v: "ASUS/AI2203/ROG Phone 6D:14/UP1A.231005.007/20240315:user/release-keys" },
        PropPair { k: "ro.build.product", v: "AI2203" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "Lenovo" },
        PropPair { k: "ro.product.device", v: "Legion Y700 (2023)" },
        PropPair { k: "ro.product.manufacturer", v: "Lenovo" },
        PropPair { k: "ro.product.model", v: "TB-9707F" },
        PropPair { k: "ro.product.fingerprint", v: "Lenovo/TB-9707F/Lenovo TB-9707F:13/TQ3A.230805.001/20230901:user/release-keys" },
        PropPair { k: "ro.product.product", v: "TB-9707F" },
        PropPair { k: "ro.build.fingerprint", v: "Lenovo/TB-9707F/Lenovo TB-9707F:13/TQ3A.230805.001/20230901:user/release-keys" },
        PropPair { k: "ro.build.product", v: "TB-9707F" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "Xiaomi" },
        PropPair { k: "ro.product.device", v: "Xiaomi 13" },
        PropPair { k: "ro.product.manufacturer", v: "Xiaomi" },
        PropPair { k: "ro.product.model", v: "2211133G" },
        PropPair { k: "ro.product.fingerprint", v: "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys" },
        PropPair { k: "ro.product.product", v: "2211133G" },
        PropPair { k: "ro.build.fingerprint", v: "Xiaomi/fuxi_eea/fuxi:13/TKQ1.221114.001/OS2.0.102.0.VMCEUXM:user/release-keys" },
        PropPair { k: "ro.build.product", v: "2211133G" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "OnePlus" },
        PropPair { k: "ro.product.device", v: "OnePlus 13" },
        PropPair { k: "ro.product.manufacturer", v: "OnePlus" },
        PropPair { k: "ro.product.model", v: "PJZ110" },
        PropPair { k: "ro.product.fingerprint", v: "OnePlus/PJZ110/OP5D0DL1:15/AP3A.240617.008/V.1bd19a1-1-2:user/release-keys" },
        PropPair { k: "ro.product.product", v: "PJZ110" },
        PropPair { k: "ro.build.fingerprint", v: "OnePlus/PJZ110/OP5D0DL1:15/AP3A.240617.008/V.1bd19a1-1-2:user/release-keys" },
        PropPair { k: "ro.build.product", v: "PJZ110" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "realme" },
        PropPair { k: "ro.product.device", v: "Realme P3 5G" },
        PropPair { k: "ro.product.manufacturer", v: "realme" },
        PropPair { k: "ro.product.model", v: "RMX5070" },
        PropPair { k: "ro.product.fingerprint", v: "realme/RMX5070/RMX5070:15/SKQ1.230119.001/eng.user.20250415.155201:user/release-keys" },
        PropPair { k: "ro.product.product", v: "RMX5070" },
        PropPair { k: "ro.build.fingerprint", v: "realme/RMX5070/RMX5070:15/SKQ1.230119.001/eng.user.20250415.155201:user/release-keys" },
        PropPair { k: "ro.build.product", v: "RMX5070" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "realme" },
        PropPair { k: "ro.product.device", v: "Realme 15 Pro 5G" },
        PropPair { k: "ro.product.manufacturer", v: "realme" },
        PropPair { k: "ro.product.model", v: "RMX5101" },
        PropPair { k: "ro.product.fingerprint", v: "realme/RMX5101IN/RE60B4L1:15/AP3A.240617.008/V.R4T2.26cec0e-80bb4e-80b757:user/release-keys" },
        PropPair { k: "ro.product.product", v: "RMX5101" },
        PropPair { k: "ro.build.fingerprint", v: "realme/RMX5101IN/RE60B4L1:15/AP3A.240617.008/V.R4T2.26cec0e-80bb4e-80b757:user/release-keys" },
        PropPair { k: "ro.build.product", v: "RMX5101" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "samsung" },
        PropPair { k: "ro.product.device", v: "Galaxy Z Fold 5" },
        PropPair { k: "ro.product.manufacturer", v: "samsung" },
        PropPair { k: "ro.product.model", v: "SM-F9460" },
        PropPair { k: "ro.product.fingerprint", v: "samsung/q2qzh/q2q:15/UP1A.231005.007/F946BXXU1BWK4:user/release-keys" },
        PropPair { k: "ro.product.product", v: "SM-F9460" },
        PropPair { k: "ro.build.fingerprint", v: "samsung/q2qzh/q2q:15/UP1A.231005.007/F946BXXU1BWK4:user/release-keys" },
        PropPair { k: "ro.build.product", v: "SM-F9460" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "HONOR" },
        PropPair { k: "ro.product.device", v: "Honor Magic V2 RSR" },
        PropPair { k: "ro.product.manufacturer", v: "HONOR" },
        PropPair { k: "ro.product.model", v: "VER-N49DP" },
        PropPair { k: "ro.product.fingerprint", v: "HONOR/VER-N49DP/VER:13/ENG.20240918.123456:user/release-keys" },
        PropPair { k: "ro.product.product", v: "VER-N49DP" },
        PropPair { k: "ro.build.fingerprint", v: "HONOR/VER-N49DP/VER:13/ENG.20240918.123456:user/release-keys" },
        PropPair { k: "ro.build.product", v: "VER-N49DP" },
    ]},
    SpoofCfg { props: &[
        PropPair { k: "ro.product.brand", v: "nubia" },
        PropPair { k: "ro.product.device", v: "RedMagic 10 Pro" },
        PropPair { k: "ro.product.manufacturer", v: "ZTE" },
        PropPair { k: "ro.product.model", v: "NX789J" },
        PropPair { k: "ro.product.fingerprint", v: "nubia/NX789J-UN/NX789J:15/AQ3A.240812.002/20241212.194919:user/release-keys" },
        PropPair { k: "ro.product.product", v: "NX789J" },
        PropPair { k: "ro.build.fingerprint", v: "nubia/NX789J-UN/NX789J:15/AQ3A.240812.002/20241212.194919:user/release-keys" },
        PropPair { k: "ro.build.product", v: "NX789J" },
    ]},
];

static PKG_TABLE: &[PkgEnt] = &[
    PkgEnt { pkg: "com.activision.callofduty.shooter", dev: 6 },
    PkgEnt { pkg: "com.activision.callofduty.warzone", dev: 0 },
    PkgEnt { pkg: "com.blitzteam.battleprime", dev: 0 },
    PkgEnt { pkg: "com.blizzard.diablo.immortal", dev: 0 },
    PkgEnt { pkg: "com.CarXTech.highWay", dev: 0 },
    PkgEnt { pkg: "com.clashoftitansandroid.india", dev: 10 },
    PkgEnt { pkg: "com.dts.freefiremax", dev: 9 },
    PkgEnt { pkg: "com.dts.freefireth", dev: 9 },
    PkgEnt { pkg: "com.ea.game.nfs14_row", dev: 0 },
    PkgEnt { pkg: "com.ea.games.r3_row", dev: 0 },
    PkgEnt { pkg: "com.ea.gp.fifamobile", dev: 11 },
    PkgEnt { pkg: "com.gamedevltd.destinywarfare", dev: 0 },
    PkgEnt { pkg: "com.gameloft.android.ANMP.GloftA8HM", dev: 0 },
    PkgEnt { pkg: "com.gameloft.android.ANMP.GloftA9HM", dev: 12 },
    PkgEnt { pkg: "com.garena.game.codm", dev: 13 },
    PkgEnt { pkg: "com.garena.game.df", dev: 0 },
    PkgEnt { pkg: "com.garena.game.kgid", dev: 10 },
    PkgEnt { pkg: "com.garena.game.kgms", dev: 10 },
    PkgEnt { pkg: "com.garena.game.kgph", dev: 10 },
    PkgEnt { pkg: "com.garena.game.kgth", dev: 10 },
    PkgEnt { pkg: "com.garena.game.kgtw", dev: 10 },
    PkgEnt { pkg: "com.garena.game.kgvn", dev: 10 },
    PkgEnt { pkg: "com.garena.game.kgvntest", dev: 10 },
    PkgEnt { pkg: "com.kurogame.wutheringwaves.global", dev: 13 },
    PkgEnt { pkg: "com.levelinfinite.aov", dev: 10 },
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
    PkgEnt { pkg: "com.ngame.allstar.eu", dev: 10 },
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
    PkgEnt { pkg: "com.tencent.aovindia", dev: 10 },
    PkgEnt { pkg: "com.tencent.aovjp", dev: 10 },
    PkgEnt { pkg: "com.tencent.ig", dev: 3 },
    PkgEnt { pkg: "com.tencent.lolm", dev: 7 },
    PkgEnt { pkg: "com.tencent.ngame.chty", dev: 10 },
    PkgEnt { pkg: "com.tencent.ngjp", dev: 10 },
    PkgEnt { pkg: "com.tencent.tmgp.gnyx", dev: 0 },
    PkgEnt { pkg: "com.tencent.tmgp.kr.codm", dev: 13 },
    PkgEnt { pkg: "com.tencent.tmgp.pubgmhd", dev: 3 },
    PkgEnt { pkg: "com.tencent.tmgp.sgame", dev: 3 },
    PkgEnt { pkg: "com.vng.codmvn", dev: 13 },
    PkgEnt { pkg: "com.vng.mlbbvn", dev: 2 },
    PkgEnt { pkg: "com.zhiliaoapp.musically", dev: 0 },
];

#[inline]
fn find_device(name: &str) -> Option<&'static SpoofCfg> {
    let t = PKG_TABLE;
    let mut l = 0usize;
    let mut h = t.len();
    while l < h {
        let m = l + (h - l) / 2;
        let e = &t[m];
        match name.as_bytes().cmp((*e).pkg.as_bytes()) {
            std::cmp::Ordering::Less => h = m,
            std::cmp::Ordering::Greater => l = m + 1,
            std::cmp::Ordering::Equal => return Some(&DEV_TABLE[(*e).dev as usize]),
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
    if let Some(cfg) = find_device(name) {
        hook::set_spoof_props(cfg.props);
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
