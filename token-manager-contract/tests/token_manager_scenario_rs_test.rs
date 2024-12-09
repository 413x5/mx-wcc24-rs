use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");
    blockchain.register_contract("mxsc:output/token-manager.mxsc.json", token_manager::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/token_manager.scen.json");
}
