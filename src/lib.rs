// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2020, Douglas Creager.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License.  You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied.  See the License for the specific language governing permissions and
// limitations under the License.
// ------------------------------------------------------------------------------------------------

use std::marker::PhantomData;

pub trait Context: Sized {
    fn add<Unit>(self, unit: Unit) -> NestedContext<Unit, Self>;
}

impl<T> Context for T {
    fn add<Unit>(self, unit: Unit) -> NestedContext<Unit, Self> {
        NestedContext {
            head: unit,
            tail: self,
        }
    }
}

pub struct NestedContext<Head, Tail> {
    pub head: Head,
    pub tail: Tail,
}

impl NestedContext<(), ()> {
    pub fn root() -> () {
        ()
    }
}

pub struct Next<T>(PhantomData<T>);

pub trait Has<Unit, Proof> {
    fn get_unit(&self) -> &Unit;
    fn get_unit_mut(&mut self) -> &mut Unit;
}

impl<Unit, Tail> Has<Unit, ()> for NestedContext<Unit, Tail> {
    fn get_unit(&self) -> &Unit {
        &self.head
    }

    fn get_unit_mut(&mut self) -> &mut Unit {
        &mut self.head
    }
}

impl<Unit, Head, Tail, TailProof> Has<Unit, Next<TailProof>> for NestedContext<Head, Tail>
where
    Tail: Has<Unit, TailProof>,
{
    fn get_unit(&self) -> &Unit {
        self.tail.get_unit()
    }

    fn get_unit_mut(&mut self) -> &mut Unit {
        self.tail.get_unit_mut()
    }
}

#[cfg(test)]
mod tests;
