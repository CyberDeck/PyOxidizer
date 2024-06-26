// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// THIS FILE IS AUTOGENERATED BY `scripts/fetch-python-distributions.py`.
// DO NOT EDIT MANUALLY.

//! Default Python distributions.

use crate::py_packaging::distribution::{PythonDistributionLocation, PythonDistributionRecord};
use crate::python_distributions::PythonDistributionCollection;
use once_cell::sync::Lazy;

pub static PYTHON_DISTRIBUTIONS: Lazy<PythonDistributionCollection> = Lazy::new(|| {
    let dists = vec![
        // Linux glibc linked.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "8df9cad9b52c6beb92d2cfae0ffda96673fc461387091293b76aae658e81e8bd".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-aarch64-unknown-linux-gnu-noopt-full.tar.zst".to_string(),
                sha256: "6203ff8773acb3d9b3ae500d3cf5935b096dfc237f3a04e5c944cf53ddceb1a4".to_string(),
            },
            target_triple: "aarch64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "b65cf8dc39c1edb6974e7a470dbcd3efa85a49c05a103c27010f5914f5906e69".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64_v2-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "d80451f83d25e3806e3ff524d2db3f87c828bf1ffdf3ce77b6588917272d86e4".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64_v3-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "b98ac7c499b9c404faf1a30480c3344d1efd905a3678453be079c66713f473c8".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-aarch64-unknown-linux-gnu-noopt-full.tar.zst".to_string(),
                sha256: "ab7195f04182d94aa675e738b6cf8affd4e897e7be7f02224cf36865194be344".to_string(),
            },
            target_triple: "aarch64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "506ea99e2cc32210aa5c409be29fed6be48bfb4a35756b7b0854c050e7924ddd".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64_v2-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "720107b4e6e53202dc18535cae5b853944759cac5d9f92786f13d47d44cb8cbc".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64_v3-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "6db6d5515677aae0c2dce500807aa01de829777ce61bed0fa1bba32b78a3c7f3".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-aarch64-unknown-linux-gnu-noopt-full.tar.zst".to_string(),
                sha256: "cde2b2c809635c2346d9d43675d75f5086f302fe40191b23b0866108ebb02ef4".to_string(),
            },
            target_triple: "aarch64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "812b818571d2edc097ed67dd0087a9d9754afcb9aebcb8643fc73be34ce2cbfe".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64_v2-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "70a0badcbdedc86b713aa4144ec5a600a7af695951c2141e5de24852e29b466c".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64_v3-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "826675bf954a1ff381d8bb20a9e5ca8d654bfda37d2a641d70e2d65bab2ac867".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-aarch64-unknown-linux-gnu-noopt-full.tar.zst".to_string(),
                sha256: "8845ffffd3bc4a87339bb4961f2025506f0143c82a73cf587c1f5eb1ccdd0fd1".to_string(),
            },
            target_triple: "aarch64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "4b7e7c7bb0daa771d6e538da1a182cbd02734c21fa3fb4da37d76ac45ae67964".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64_v2-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "1f69260caf30a83879ebfcb6f5bf13d2188f1a23f05d83dbe36c8332100505ef".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64_v3-unknown-linux-gnu-pgo-full.tar.zst".to_string(),
                sha256: "ff855f631ce112bb84b5878aac4b0adfb14c3e8f61b5df72acdd0c336c25e754".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Linux musl.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "e4e00f7c4b2b8ae690bea40de0e72cb34f10c804a59682ecabaf17a6d46bbcdb".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "95c54245998cec089261b09e558a330d7231f025b9b4d41c858a1fb74c4b7e68".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64_v2-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "23078a155029cb6406847ab069c572e34ef777b3736dce30cbd4bcd01fcec0b9".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64_v3-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "e35db6160637b89604e82db327444e85ea5fb8c24dd1c24bce1bd33d74e59633".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "e711f6a063efb665e425f4f071c75f2a8c107f599c99130ecfff2b0532f7b804".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64_v2-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "132ba0b3b4da2d38f2b5af30e55d951badf0cca80ee24ea5f92face7fe291eae".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64_v3-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "b31f427e49ecbac5a54088c2b7a5da53be1df9c6a94d696d0d71400a8853d34e".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "25f4d705ee2323dba9794643be08796c2dc2455edbba808274fb2752c5601af9".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64_v2-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "edeeb6d5f705f9398bad6dde32fbdd4c793d5df9361f1b659579b2c787f5df72".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64_v3-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "adc4048ceadd40901299847a2b832974303fe5ed0977ef5334c1c9fdb3121d0b".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "1c047cd7323e64cb610e24a747eedf14289ead687ec45ddb92165ce00d085477".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64_v2-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "12da018a18c08faf473dac26c326d94637d447e08080c685eeb9614880160742".to_string(),
            },
            target_triple: "x86_64_v2-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64_v3-unknown-linux-musl-noopt-full.tar.zst".to_string(),
                sha256: "3e77902d720c2aa856584b376d2d23ff05be8990739a99efeae25b66e26c4635".to_string(),
            },
            target_triple: "x86_64_v3-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // The order here is important because we will choose the
        // first one. We prefer shared distributions on Windows because
        // they are more versatile: statically linked Windows distributions
        // don't declspec(dllexport) Python symbols and can't load shared
        // shared library Python extensions, making them a pain to work
        // with.

        // Windows shared.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "9f94c7b54b97116cd308e73cda0b7a7b7fff4515932c5cbba18eeae9ec798351".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "212d413ab6f854f588cf368fdd2aa140bb7c7ee930e3f7ac1002cba1e50e9685".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "c8b99dcf267c574fdfbdf4e9d63ec7a4aa4608565fee3fba0b2f73843b9713b2".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "c3e90962996177a027bd73dd9fd8c42a2d6ef832cda26db4ab4efc6105160537".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-i686-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "ee985ae6a6a98f4d5bd19fd8c59f45235911d19b64e1dbd026261b8103f15db5".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "c63abd9365a13196eb9f65db864f95b85c1f90b770d218c1acd104e6b48a99d3".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "924ed4f375ef73c73a725ef18ec6a72726456673d5a116f132f60860a25dd674".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "6a2c8f37509556e5d463b1f437cdf7772ebd84cdf183c258d783e64bb3109505".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "6da82390f7ac49f6c4b19a5b8019c4ddc1eef2c5ad6a2f2d32773a27663a4e14".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64-pc-windows-msvc-shared-pgo-full.tar.zst".to_string(),
                sha256: "a1daf5e8ceb23d34ea29b16b5123b06694810fe7acc5c8384426435c63bf731e".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Windows static.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "1d4cf41c9ec4d4ba5a7de6510059bbb071e7764afd974391ec2bb632f86847f7".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "4f967b85d50258112ae7928ae6ee222fddeb043329edfaa41bbd8f4f1ce821b1".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "21e85cfd7bfd658dbf2377334f78f1568370f4041a524af7e718cb45903fecef".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "2c504dba00390a3a3bc77fda0700282183e90fe7e91ee6f2b1b0cf59db07643b".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-i686-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "9b8a75b7d662a4ac6b141b6d1c527806c133c5d0d95c3394adca90c14470ebe5".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "72167680665c8d8db0fa7b9415ef94c065c2f984b53ecf470b2a06e0372309d8".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "aef10b165b10dc6eee599ce734eeee51717864a668f0fe9f224014577930dab2".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "7b0523a2ab5e801a40e9467aa96f8137ca9b26b9f6c91d57de457c24d5557f96".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "40d7981e8409669dd6e6b6ea4784afd71917f87bdbf1d856334b12a44cd6756d".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64-pc-windows-msvc-static-noopt-full.tar.zst".to_string(),
                sha256: "e037ede1f5f5a5c7d44a8514d63cc09f5d0e5ee61f14fce7726309a941620c2f".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },

        // macOS.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-aarch64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "eea082f36739911147da65558e88896cd929fd1c0c1331a83f03378cc1453ded".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-aarch64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "908526e6bdc4393ddb8e44f4246c453b338fdca2ced23ff14e575e6909ca7b58".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-aarch64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "5f90a26379f423de40c1be6c06fbc68b82f8b09f43e657932a48df30d3f5dba4".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-aarch64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "df7d8f3012dc653eed9e1b5f98d5f623093594dae5d88ea600f6d66fb9421937".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-aarch64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "4a7c07abb5d59df4b9ba1702b142bef1475a8be0c539fa7fdd1a48df5fd58321".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.8.18%2B20240224-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "ef270d19db99bbaf967954a9fad5ed7761a5f2d9c6132ee5853fbd6d4e66418e".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.9.18%2B20240224-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "a38d2c6dde8c903520293805e8d5ac858b812394975f1959cac050f49621cea5".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.10.13%2B20240224-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "178dda6f63f8bf9438649743ab659f47f5f379b36b6fcc51491f49c8c01f4615".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.11".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.11.8%2B20240224-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "aa1fa172d22bed284350b1d95653358826b4bc560745eb2ab153b69db7356e28".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.12".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20240224/cpython-3.12.2%2B20240224-x86_64-apple-darwin-pgo-full.tar.zst".to_string(),
                sha256: "2fd179e98ceaed2b3e419f9c2aa669613e53d871bb42074bcef460f5bcc1f4fb".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
    ];

    PythonDistributionCollection { dists }
});
