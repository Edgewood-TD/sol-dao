import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolDev } from "../target/types/sol_dev";
import { getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import assert from "assert";
const { SystemProgram } = anchor.web3;
const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
describe("sol-dev", () => {
	const provider = anchor.Provider.env();

	anchor.setProvider(provider);

	// Configure the client to use the local cluster.

	anchor.setProvider(anchor.Provider.env());
	const daoKey = anchor.web3.Keypair.generate();
	const account2 = anchor.web3.Keypair.generate();
	const program = anchor.workspace.SolDev as Program<SolDev>;
	const NFT_CREATOR_PUBKEY = new anchor.web3.PublicKey("D7yhCX4ufWJmw2aeb2ZW6jYhy2Uofjkb13LF7FBStusj");
	const TEST_NFT_MINT = new anchor.web3.PublicKey("Pv7cvP4HmAgXNNYu8bW7CxWkRJMEc2DXsoW5Y2kbTUD");
	const FALSE_NFT_MINT = new anchor.web3.PublicKey("BGnZBc1PhNb93aQPaV7B2UYQnN5zdLz1HpWgL7QCErAG");
	const metaplex_pubKey = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
	it("Is initialized!", async () => {
		// Add your test here.

		const [daoPDA, _daoBump] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from("dao"), provider.wallet.publicKey.toBuffer()],
			program.programId
		);
		/* 	await program.rpc.initDao("First DAO", {
			accounts: {
				dao: daoPDA,
				daoManager: provider.wallet.publicKey,

				systemProgram: SystemProgram.programId,
			},
		});  

		await program.rpc.configDao(NFT_CREATOR_PUBKEY, {
			accounts: {
				dao: daoPDA,
				daoManager: provider.wallet.publicKey,
				systemProgram: SystemProgram.programId,
			},
		}); */
		const proposal = anchor.web3.Keypair.generate();
		const [metadata_pda, _bump_meta] = await anchor.web3.PublicKey.findProgramAddress(
			[Buffer.from("metadata"), metaplex_pubKey.toBuffer(), TEST_NFT_MINT.toBuffer()],
			metaplex_pubKey
		);
		const nftTokenAccount = await getAssociatedTokenAddress(
			TEST_NFT_MINT,
			provider.wallet.publicKey,
			false,
			TOKEN_PROGRAM_ID,
			ASSOCIATED_TOKEN_PROGRAM_ID
		);
		await program.rpc.createProposal("POPOPO2 URL", {
			accounts: {
				dao: daoPDA,
				proposal: proposal.publicKey,
				proposer: provider.wallet.publicKey,
				nftMetadataAccount: metadata_pda,
				nftMint: TEST_NFT_MINT,
				nftAccount: nftTokenAccount,
				systemProgram: anchor.web3.SystemProgram.programId,
			},
			signers: [proposal],
		});
		const daoName = await program.account.dao.fetch(daoPDA);
		const proposal_details_pre_remove = await program.account.proposal.fetchMultiple(daoName.proposals);
		console.log(daoName.proposals);
		await program.rpc.removeProposal({
			accounts: {
				dao: daoPDA,
				proposer: provider.wallet.publicKey,
				systemProgram: anchor.web3.SystemProgram.programId,
				proposal: daoName.proposals[0],
			},
		});

		const proposal_details_after_remove = await program.account.proposal.fetchMultiple(daoName.proposals);
		console.log(daoName.proposals);
		console.log("REMOVED:", daoName.proposals[0]);
	});
});
