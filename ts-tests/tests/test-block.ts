import { expect } from "chai";
import { step } from "mocha-steps";
import { describeWithSelendra } from "./util";

describeWithSelendra("Selendra RPC (Block)", (context) => {
	step("should be at block 0 at genesis", async function () {
		expect(await context.provider.getBlockNumber()).to.equal(0);
	});

	it("should return genesis block by number", async function () {
		expect(await context.provider.getBlockNumber()).to.equal(0);
	});
});
