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

pub struct Context<Stack>(Stack);

impl Context<()> {
    pub fn root() -> () {
        ()
    }
}

impl<Stack> Context<Stack> {
    pub fn stack(&self) -> &Stack {
        &self.0
    }
}

pub trait ContextParent: Sized {
    fn add<Next>(self, next: Next) -> ContextStack<Next, Self>;
    fn seal(self) -> Context<Self>;
}

impl ContextParent for () {
    fn add<Next>(self, next: Next) -> ContextStack<Next, Self> {
        ContextStack {
            head: next,
            tail: self,
        }
    }

    fn seal(self) -> Context<Self> {
        Context(self)
    }
}

impl<H, T> ContextParent for ContextStack<H, T> {
    fn add<Next>(self, next: Next) -> ContextStack<Next, Self> {
        ContextStack {
            head: next,
            tail: self,
        }
    }

    fn seal(self) -> Context<Self> {
        Context(self)
    }
}

pub struct ContextStack<Head, Tail> {
    pub head: Head,
    pub tail: Tail,
}

pub struct Next<T>(PhantomData<T>);

pub trait Has<Unit, Proof> {
    fn get_unit(&self) -> &Unit;
    fn get_unit_mut(&mut self) -> &mut Unit;
}

impl<Stack, Unit, Proof> Has<Unit, Proof> for Context<Stack>
where
    Stack: Has<Unit, Proof>,
{
    fn get_unit(&self) -> &Unit {
        self.0.get_unit()
    }

    fn get_unit_mut(&mut self) -> &mut Unit {
        self.0.get_unit_mut()
    }
}

impl<Unit, Tail> Has<Unit, ()> for ContextStack<Unit, Tail> {
    fn get_unit(&self) -> &Unit {
        &self.head
    }

    fn get_unit_mut(&mut self) -> &mut Unit {
        &mut self.head
    }
}

impl<Unit, Head, Tail, TailProof> Has<Unit, Next<TailProof>> for ContextStack<Head, Tail>
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
