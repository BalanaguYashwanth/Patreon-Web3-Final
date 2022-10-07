sample.txt


// let accountPrivKey=[9,211,148,158,204,187,155,143,229,24,195,4,21,149,126,128,65,145,238,107,194,70,145,99,192,193,74,221,81,0,145,33,241,181,234,185,158,153,151,97,199,236,144,223,252,162,190,54,213,50,161,16,123,73,233,206,206,194,174,176,153,201,225,122].slice(0,32);
// let User_Wallet = web3.Keypair.fromSeed(Uint8Array.from(accountPrivKey));

// let getBinaryFromWalletAddress = new PublicKey('6xxbFNeTtygiMtYXQEy846n5U9Q6bTmwFtUmje9kBHoS')
// let User_Wallet = web3.Keypair.fromSeed(Uint8Array.from(getBinaryFromWalletAddress));


// let getBinaryFromWalletAddress = new PublicKey('6xxbFNeTtygiMtYXQEy846n5U9Q6bTmwFtUmje9kBHoS')
// console.log('patreonNewkeyPair',getBinaryFromWalletAddress, 'patreonNewkeyPair user wallet' )


 const initializaTokenPDA = async() =>{
    let transaction = new web3.Transaction()
    const provider = getProvider()
    const program = new Program(idl,programID,provider)

    let mintA = await createMint(connection, patreonNewkeyPair,patreonNewkeyPair.publicKey, null, 0);
    console.log('mintA',mintA)
    // let myToken_acctA = await getAssociatedTokenAddress(patreonNewkeyPair.publicKey,walletAddress)
    // console.log('myToken_acctA',myToken_acctA)
    let myToken_acctA = await getOrCreateAssociatedTokenAccount(connection,patreonNewkeyPair,mintA,patreonNewkeyPair.publicKey)
    await mintTo(connection,patreonNewkeyPair,mintA,myToken_acctA.address,patreonNewkeyPair.publicKey,5)
    let amount =1;

     // state PDA for token
    const [user_pda_state, bump_state] = await web3.PublicKey.findProgramAddress(
      [ provider?.wallet?.publicKey?.toBuffer(),myToken_acctA.address?.toBuffer(),Buffer.from("state")],
      programID
    );


    if(await connection.getAccountInfo(user_pda_state)==null){
      transaction.add(await program.methods.initializestatepda(bump_state)
      .accounts({
        statepda:user_pda_state,
        owner:walletAddress,
        depositTokenAccount:myToken_acctA.address,
        systemProgram: SystemProgram.programId
      }).signers([patreonNewkeyPair])
      .instruction())
    }
    await sendAndConfirmTransaction(connection,transaction,[patreonNewkeyPair]);
    

  }
