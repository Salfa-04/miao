let chip = switch variable::get("board") {

    "RM-TYPEC" => {
        variable::set("rm_typec", true);
        variable::set("ocdcfg", "stm32f4x.cfg");

        "stm32f407ig"
    },

    "DM-MC02" => {
        variable::set("dm_mc02", true);
        variable::set("ocdcfg", "stm32h7x.cfg");

        "stm32h723vg"
    },

    _ => variable::get("chip"),
};

let chip_l = chip.to_lower();
let chip_u = chip.to_upper();

// Safety: Regex Ensures.
let prefix = chip_l.sub_string(..7);

let board2chip = #{
    "RM-TYPEC": "stm32f407ig",
    "DM-MC02":  "stm32h723vg",
};

let chip2core = #{
    "stm32u0": "Cortex-M0(+)",
    "stm32l0": "Cortex-M0(+)",
    "stm32c0": "Cortex-M0(+)",
    "stm32f0": "Cortex-M0(+)",
    "stm32g0": "Cortex-M0(+)",

    "stm32l1": "Cortex-M3",
    "stm32f1": "Cortex-M3",
    "stm32f2": "Cortex-M3",

    "stm32l4": "Cortex-M4F/M7F",
    "stm32f3": "Cortex-M4F/M7F",
    "stm32g4": "Cortex-M4F/M7F",
    "stm32f4": "Cortex-M4F/M7F",

    "stm32l5": "Cortex-M33F",
    "stm32u3": "Cortex-M33F",
    "stm32u5": "Cortex-M33F",
    "stm32h5": "Cortex-M33F",

    "stm32f7": "Cortex-M4F/M7F",
    "stm32h7": "Cortex-M4F/M7F",
};

let core2target = #{
    "Cortex-M0(+)":     "thumbv6m-none-eabi",
    "Cortex-M3":        "thumbv7m-none-eabi",
    "Cortex-M4/M7":     "thumbv7em-none-eabi",
    "Cortex-M4F/M7F":   "thumbv7em-none-eabihf",
    "Cortex-M23":       "thumbv8m.base-none-eabi",
    "Cortex-M33":       "thumbv8m.main-none-eabi",
    "Cortex-M33F":      "thumbv8m.main-none-eabihf",
};

if prefix !in chip2core {
    abort("Unsupported Chip: " + prefix);
}

let core = chip2core.get(prefix);
let target = core2target.get(core);

variable::set("target", target);
variable::set("chip_l", chip_l);
variable::set("chip_u", chip_u);

// print("Detected MCU: " + chip);
// print("Using target: " + target);
// print("Crate Name: " + variable::get("crate_name"));
// print("Crate Type: " + variable::get("crate_type"));
// print("Project Name: " + variable::get("project-name"));
// print("Author: " + variable::get("authors"));
// print("User Name: " + variable::get("username"));
// print("Os-Arch: " + variable::get("os-arch"));
// print(variable::get("within_cargo_project"));
// print(variable::get("is_init"));
