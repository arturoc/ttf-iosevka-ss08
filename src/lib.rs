// The font is licensed under SIL OFL Version 1.1.
//
// The support code is licensed under Berkeley Software Distribution license.
//
// ---
// ---
//
// Copyright (c) 2015-2017 Belleve Invis (belleve@typeof.net).
//
// Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
// * Neither the name of Belleve Invis nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL BELLEVE INVIS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// -----------------------


#[cfg(feature = "regular")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-regular.ttf");

#[cfg(feature = "bold")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-bold.ttf");

#[cfg(feature = "italic")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-italic.ttf");

#[cfg(feature = "oblique")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-oblique.ttf");

#[cfg(feature = "bolditalic")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-bolditalic.ttf");

#[cfg(feature = "boldoblique")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-boldoblique.ttf");

#[cfg(feature = "thin")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-thin.ttf");

#[cfg(feature = "thinitalic")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-thinitalic.ttf");

#[cfg(feature = "thinoblique")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-thinoblique.ttf");

#[cfg(feature = "light")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-light.ttf");

#[cfg(feature = "lightitalic")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-lightitalic.ttf");

#[cfg(feature = "lightoblique")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-lightoblique.ttf");

#[cfg(feature = "medium")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-medium.ttf");

#[cfg(feature = "mediumitalic")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-mediumitalic.ttf");

#[cfg(feature = "mediumoblique")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-mediumoblique.ttf");

#[cfg(feature = "heavy")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-heavy.ttf");

#[cfg(feature = "heavyitalic")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-heavyitalic.ttf");

#[cfg(feature = "heavyoblique")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-heavyoblique.ttf");

#[cfg(feature = "extralight")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-extralight.ttf");

#[cfg(feature = "extralightitalic")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-extralightitalic.ttf");

#[cfg(feature = "extralightoblique")]
pub const REGULAR: &'static [u8] = include_bytes!("../ttf/iosevka-ss08-extralightoblique.ttf");
