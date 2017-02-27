// Copyright 2017 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Authentication modules.
//!
//! Usually, accessing OpenStack services requires authentication. This module
//! provides a way to authenticate against an Identity service, as well as
//! simple authentication implementations for standalone use.

pub mod base;
pub mod identity;

pub use self::base::{AuthError, AuthMethod, NoAuth};
pub use self::identity::Identity;
