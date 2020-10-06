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

use crate::Context;
use crate::Has;
use crate::NestedContext;

struct FirstName {
    name: String,
}

impl FirstName {
    fn new<C, S>(ctx: C, name: S) -> NestedContext<Self, C>
    where
        C: Context,
        S: ToString,
    {
        ctx.add(FirstName {
            name: name.to_string(),
        })
    }
}

trait HasFirstName<Proof> {
    fn first_name(&self) -> &String;
}

impl<T, Proof> HasFirstName<Proof> for T
where
    T: Has<FirstName, Proof>,
{
    fn first_name(&self) -> &String {
        &self.get_unit().name
    }
}

#[test]
fn can_add_first_name() {
    let ctx = NestedContext::root();
    let ctx = FirstName::new(ctx, "Rusty");
    assert_eq!(ctx.first_name(), "Rusty");
}

struct LastName {
    name: String,
}

impl LastName {
    fn new<C, S>(ctx: C, name: S) -> NestedContext<Self, C>
    where
        C: Context,
        S: ToString,
    {
        ctx.add(LastName {
            name: name.to_string(),
        })
    }
}

trait HasLastName<Proof> {
    fn last_name(&self) -> &String;
}

impl<T, Proof> HasLastName<Proof> for T
where
    T: Has<LastName, Proof>,
{
    fn last_name(&self) -> &String {
        &self.get_unit().name
    }
}

#[test]
fn can_add_last_name() {
    let ctx = NestedContext::root();
    let ctx = LastName::new(ctx, "McRustface");
    assert_eq!(ctx.last_name(), "McRustface");
}

#[test]
fn can_add_both_names() {
    let ctx = NestedContext::root();
    let ctx = FirstName::new(ctx, "Rusty");
    let ctx = LastName::new(ctx, "McRustface");
    assert_eq!(ctx.first_name(), "Rusty");
    assert_eq!(ctx.last_name(), "McRustface");
}

trait HasFullName<FirstNameProof, LastNameProof> {
    fn full_name(&self) -> String;
}

impl<T, FirstNameProof, LastNameProof> HasFullName<FirstNameProof, LastNameProof> for T
where
    T: Has<FirstName, FirstNameProof>,
    T: Has<LastName, LastNameProof>,
{
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name(), self.last_name())
    }
}

#[test]
fn can_get_full_name() {
    let ctx = NestedContext::root();
    let ctx = FirstName::new(ctx, "Rusty");
    let ctx = LastName::new(ctx, "McRustface");
    assert_eq!(ctx.full_name(), "Rusty McRustface");
}

trait Registry {
    fn register(&self, units: &mut Vec<String>);
}

impl Registry for () {
    fn register(&self, _units: &mut Vec<String>) {}
}

impl Registry for FirstName {
    fn register(&self, units: &mut Vec<String>) {
        units.push("FirstName".to_string());
    }
}

impl Registry for LastName {
    fn register(&self, units: &mut Vec<String>) {
        units.push("LastName".to_string());
    }
}

impl<Head, Tail> Registry for NestedContext<Head, Tail>
where
    Head: Registry,
    Tail: Registry,
{
    fn register(&self, units: &mut Vec<String>) {
        self.head.register(units);
        self.tail.register(units);
    }
}

#[test]
fn can_register_first_name() {
    let ctx = NestedContext::root();
    let ctx = FirstName::new(ctx, "Rusty");
    let mut registry = Vec::new();
    ctx.register(&mut registry);
    assert_eq!(registry, vec!["FirstName"]);
}

#[test]
fn can_register_last_name() {
    let ctx = NestedContext::root();
    let ctx = LastName::new(ctx, "McRustface");
    let mut registry = Vec::new();
    ctx.register(&mut registry);
    assert_eq!(registry, vec!["LastName"]);
}

#[test]
fn can_register_both_names() {
    let ctx = NestedContext::root();
    let ctx = FirstName::new(ctx, "Rusty");
    let ctx = LastName::new(ctx, "McRustface");
    let mut registry = Vec::new();
    ctx.register(&mut registry);
    registry.sort();
    assert_eq!(registry, vec!["FirstName", "LastName"]);
}
