use std::sync::Arc;

use crate::context::RuleContext;
use crate::signals::{RuleAction, RuleDiagnostic, RuleSignal};

pub trait Rule: Send + Sync {
    type State: Send;

    fn name(&self) -> &'static str;

    fn run<'a>(&self, ctx: &RuleContext<'a>) -> Vec<Self::State>;

    fn diagnostic(&self, ctx: &RuleContext, state: &Self::State) -> RuleDiagnostic;

    fn action(&self, _ctx: &RuleContext, _state: &Self::State) -> Option<RuleAction> {
        None
    }

    fn signals<'a>(&self, ctx: &RuleContext<'a>) -> Vec<RuleSignal> {
        self.run(ctx)
            .into_iter()
            .map(|state| {
                let diagnostic = self.diagnostic(ctx, &state);
                let action = self.action(ctx, &state);
                RuleSignal { diagnostic, action }
            })
            .collect()
    }

    fn is_typescript_only(&self) -> bool {
        false
    }

    fn is_regex_only(&self) -> bool {
        false
    }

    fn is_fixable(&self) -> bool {
        false
    }
}

pub trait DynRule: Send + Sync {
    fn name(&self) -> &str;
    fn signals<'a>(&self, ctx: &RuleContext<'a>) -> Vec<RuleSignal>;
    fn is_typescript_only(&self) -> bool;
    fn is_regex_only(&self) -> bool;
    fn is_fixable(&self) -> bool;
}

impl<T: Rule> DynRule for T {
    fn name(&self) -> &str {
        Rule::name(self)
    }

    fn signals<'a>(&self, ctx: &RuleContext<'a>) -> Vec<RuleSignal> {
        Rule::signals(self, ctx)
    }

    fn is_typescript_only(&self) -> bool {
        Rule::is_typescript_only(self)
    }

    fn is_regex_only(&self) -> bool {
        Rule::is_regex_only(self)
    }

    fn is_fixable(&self) -> bool {
        Rule::is_fixable(self)
    }
}

pub struct RuleRegistration {
    pub name: &'static str,
    pub factory: fn(Option<&serde_json::Value>) -> Arc<dyn DynRule>,
}

inventory::collect!(RuleRegistration);
