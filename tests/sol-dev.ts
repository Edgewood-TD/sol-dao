import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolDev } from "../target/types/sol_dev";
import assert from "assert";
const { SystemProgram } = anchor.web3;
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
describe("sol-dev", () => {
	const provider = anchor.Provider.local();
	anchor.setProvider(provider);

	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.Provider.env());
	const daoKey = anchor.web3.Keypair.generate();
	const account2 = anchor.web3.Keypair.generate();
	const program = anchor.workspace.SolDev as Program<SolDev>;

	it("Is initialized!", async () => {
		// Add your test here.
		await provider.connection.requestAirdrop(account2.publicKey, anchor.web3.LAMPORTS_PER_SOL * 2);
		await program.rpc.initDao("First DAO", {
			accounts: {
				dao: daoKey.publicKey,
				daoManager: provider.wallet.publicKey,
				payer: provider.wallet.publicKey,
				systemProgram: SystemProgram.programId,
			},
			signers: [daoKey],
		});

		const [memberPDA, _bump] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from("member"), provider.wallet.publicKey.toBuffer(), daoKey.publicKey.toBuffer()],
			program.programId
		);

		/* await program.rpc.initMember({
			accounts: {
				dao: daoKey.publicKey,
				member: memberPDA,
				creator: provider.wallet.publicKey,
				payer: provider.wallet.publicKey,
				systemProgram: SystemProgram.programId,
			},
		}); */

		const daoName = await program.account.dao.fetch(daoKey.publicKey);
		console.log(daoName);
		/* 		const memberDets = await program.account.member.fetch(memberPDA);
		console.log(memberDets); */
	});
});
