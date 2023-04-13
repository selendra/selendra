// Copyright 2023 Smallworld Selendra
// This file is part of Selendra.

// Selendra is free software: you can redistribute it and/or modify
// it under the terms of the Apache License as published by
// the Free Software Foundation

// Selendra is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Apache License for more details.

// You should have received a copy of the Apache License
// along with Selendra.  If not, see <https://www.apache.org/licenses/LICENSE-2.0>.

use sp_std::vec::Vec;

pub mod v0_to_v1;
pub mod v1_to_v2;
pub mod v2_to_v3;

type Validators<T> = Vec<<T as frame_system::Config>::AccountId>;