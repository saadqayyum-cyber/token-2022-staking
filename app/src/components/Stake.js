import { AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { useEffect, useState } from "react";
import idl from "../idl/idl.json";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { Audio } from "react-loader-spinner";
import Swal from "sweetalert2";

const STAKING_CONTRACT_ADDRESS = process.env.REACT_APP_STAKING_CONTRACT_ADDRESS;
const TOKEN_ADDRESS = process.env.REACT_APP_TOKEN_ADDRESS;
const programID = STAKING_CONTRACT_ADDRESS || null;
const tokenMintAddress = TOKEN_ADDRESS ? new PublicKey(TOKEN_ADDRESS) : null;

function Stake() {
  const { connection } = useConnection();
  const wallet = useWallet();
  const provider = new AnchorProvider(connection, wallet, "processed");

  const [totalStaked, setTotalStaked] = useState(0);
  const [yourStaked, setYourStaked] = useState(0);
  const [stakeAmount, setStakeAmount] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  const [transactionHistory, setTransactionHistory] = useState([]);
  const [rewards, setRewards] = useState(0);

  let program = null;

  if (programID) {
    program = new Program(idl, programID, provider);
  }

  useEffect(() => {
    if (wallet.connected && program) {
      fetchStakingInfo();
      fetchTransactionHistory();
    }
  }, [wallet.connected]);

  const fetchStakingInfo = async () => {
    setIsLoading(true);
    try {
      // Fetch total staked and user's staked amount from the contract
      // Assume we have methods to get these values
      const total = await program.account.stakePool.totalStaked();
      const yourStake = await program.account.stakePool.yourStaked(wallet.publicKey);
      setTotalStaked(total);
      setYourStaked(yourStake);
      // Fetch rewards
      const reward = await program.account.stakePool.calculateRewards(wallet.publicKey);
      setRewards(reward);
    } catch (error) {
      console.error("Error fetching staking info:", error);
      Swal.fire("Error", "Failed to fetch staking info", "error");
    }
    setIsLoading(false);
  };

  const fetchTransactionHistory = async () => {
    try {
      // Fetch user's transaction history from the contract or blockchain
      const history = await program.account.stakePool.transactionHistory(wallet.publicKey);
      setTransactionHistory(history);
    } catch (error) {
      console.error("Error fetching transaction history:", error);
      Swal.fire("Error", "Failed to fetch transaction history", "error");
    }
  };

  const handleStake = async () => {
    setIsLoading(true);
    try {
      const tx = await program.methods
        .stake(new BN(stakeAmount * LAMPORTS_PER_SOL))
        .accounts({
          authority: wallet.publicKey,
          tokenMint: tokenMintAddress,
        })
        .signers([wallet])
        .rpc();
      console.log("Transaction signature:", tx);
      Swal.fire("Success", "Stake successful", "success");
      fetchStakingInfo();
    } catch (error) {
      console.error("Error staking:", error);
      Swal.fire("Error", "Stake failed", "error");
    }
    setIsLoading(false);
  };

  return (
    <div className="flex flex-col items-center justify-center h-screen px-4">
      <div className="bg-purple-light rounded-lg p-6 w-full max-w-md mx-auto">
        <div className="flex justify-between items-center">
          <h2 className="text-xl font-semibold">Stake Pool</h2>
          {wallet.connected && <span className="text-xs text-gray-500">Wallet: {wallet.publicKey.toBase58()}</span>}
        </div>
        <div className="mt-4">
          <div className="bg-purple-medium p-4 rounded-lg">
            <div className="flex justify-between">
              <span>Your Staked</span>
            </div>
            <div className="text-3xl font-bold">{yourStaked} TOKENS</div>
          </div>
          <div className="mt-4">
            <div className="flex justify-between">
              <span className="text-sm">Total Locked</span>
            </div>
            <div className="text-xl">{totalStaked} TOKENS</div>
          </div>
          <div className="mt-4">
            <div className="flex justify-between">
              <span className="text-sm">Rewards</span>
            </div>
            <div className="text-xl">{rewards} TOKENS</div>
          </div>
          <div className="mt-4">
            <label htmlFor="amount" className="block">
              AMOUNT TO STAKE
            </label>
            <div className="flex items-center mt-1">
              <input
                id="amount"
                type="number"
                value={stakeAmount}
                onChange={(e) => setStakeAmount(e.target.value)}
                className="bg-purple-medium p-2 rounded-l-lg w-full"
              />
            </div>
          </div>
          <div className="mt-6 flex flex-col sm:flex-row justify-center space-y-4 sm:space-y-0">
            {isLoading ? (
              <Audio height="35" width="35" radius="9" color="yellow" ariaLabel="loading" wrapperStyle wrapperClass />
            ) : (
              <button className="bg-yellow-400 text-black px-6 py-2 rounded-lg" onClick={handleStake}>
                STAKE
              </button>
            )}
          </div>
        </div>
        <div className="mt-6">
          <h3 className="text-lg font-semibold">Transaction History</h3>
          <div className="bg-purple-medium p-4 rounded-lg mt-2">
            {transactionHistory.length > 0 ? (
              transactionHistory.map((tx, index) => (
                <div key={index} className="text-sm">
                  {tx}
                </div>
              ))
            ) : (
              <div className="text-sm text-gray-500">No transactions found</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default Stake;
