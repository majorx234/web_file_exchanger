/*
 * This file is part of the web_file_exchanger distribution (https://github.com/majorx234/web_file_exchanger ).
 * Copyright (c) 2023-2024 Majorx234 <majorx234@googlemail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::file_indexer::FileIndex;
use std::boxed::Box;
use std::sync::Arc;

use crate::database::DataBaseInterface;

pub struct ServerElements {
    pub dbi: Box<dyn DataBaseInterface + Send + Sync + 'static>,
    pub file_index: Box<FileIndex>,
}
impl ServerElements {
    pub fn new(
        dbi: Box<dyn DataBaseInterface + Send + Sync>,
        file_index: Box<FileIndex>,
    ) -> ServerElements {
        ServerElements { dbi, file_index }
    }
}

pub type ServerState = Arc<ServerElements>;
