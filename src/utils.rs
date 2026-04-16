/*
Copyright (C) 2026 leoflo

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use chrono::{DateTime, Local, NaiveDateTime};

pub(super) fn current_naive_date_time() -> NaiveDateTime {
    return DateTime::parse_from_rfc3339(Local::now().to_rfc3339().as_str())
        .unwrap()
        .naive_local();
}
