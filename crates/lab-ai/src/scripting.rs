/*use rhai::{Engine, EvalAltResult, Scope};

#[derive(Clone)]
pub struct ScriptResult {
    result_string : String
}
#[allow(dead_code)]
pub struct ScriptHost;

impl ScriptHost {
    #[allow(dead_code)]
    pub fn run(script : &str) -> Result<ScriptResult, Box<EvalAltResult>> {
        let mut engine = Engine::new();

        engine.register_type::<ScriptResult>();

        let mut scope = Scope::new();

        engine.eval_with_scope::<ScriptResult>(&mut scope, script)
    }
}*/
