import { Connection, programs } from '@metaplex/js';
import { PublicKey, Keypair, sendAndConfirmTransaction } from '@solana/web3.js';
import { scheduleJob, gracefulShutdown } from 'node-schedule';

const k = new Keypair();
console.log(k.secretKey);

//=========================== UPDATE NFT METADATA VIA SIMPLE FUNCTION CALLS =======================

//updates the nft metadata - uri to another image
const updateMetadata1 =  async function () {
  await updateMetadata("https://ipfs.io/ipfs/QmUSbZNAjE2zG3EoFrmyhsEzQZyjWRoDhsnYK2fmnkWWJG")
};
const uri_update = await updateMetadata1();
console.log("Result",uri_update);

//updates the nft uri and symbol 
const updateMetadata2 =  async function (symbol) {
  await updateMetadata("https://ipfs.io/ipfs/QmR8aXTiY4W4KKvhyQPzFAPvqSdXQsWj1pXvVsxkKQ89uQ")
};
const symbol_res = await updateMetadata2("SIGN");
console.log("Result",symbol_res);


//========================== UPDATE NFT METADATA VIA CRON JOBS ===================================

//This will execute a cron job when the minute is 42 (e.g. 19:42, 20:42, etc.)
const job = scheduleJob('42 * * * *', async function(){
  const res = await updateMetadata("https://ipfs.io/ipfs/QmR8aXTiY4W4KKvhyQPzFAPvqSdXQsWj1pXvVsxkKQ89uQ");
  console.log("NFT updated");
});

const updateMetadata = async (newUri,symbol=undefined) => {
  let { metadata : {Metadata, UpdateMetadata, MetadataDataData, Creator} } = programs;
  
  let conn = new Connection('devnet');
  let signer = Keypair.fromSeed([
    230,  14,  11, 101, 163, 218, 101, 207, 153,  90, 102,
    104, 114, 158,  37,  68,  22,  76, 218, 104, 205, 124,
    113, 202, 249,  46, 162,   4, 159, 116,   9, 213,  15,
    203, 158, 166, 188, 101, 168, 211, 191, 169, 182,   3,
    73,  13,  29,  74, 151, 154,  14, 131, 200,  78,  95,
    129,  40, 133, 223, 113, 229, 177, 103,  18
  ]);
  let acc = new PublicKey("81nDRT8AvWTWK2ANw7gvMfjDbBfGTQqqKmnnULNoMeUe");
  
  let metadataAccount = await Metadata.getPDA(acc);
  //get current metadata
  const curr_metadata = await Metadata.load(conn, metadataAccount);
  if (curr_metadata.data.data.creators != null) {
    const creators = curr_metadata.data.data.creators.map(
      (el) =>
          new Creator({
              ...el,
          }),
    );

    //update nft metadata
    let newMetadataData = new MetadataDataData({
      name: curr_metadata.data.data.name,
      symbol: symbol!==undefined?symbol:curr_metadata.data.data.symbol,
      uri: newUri,
      creators: [...creators],
      sellerFeeBasisPoints: curr_metadata.data.data.sellerFeeBasisPoints,
    })

    const updateTx = new UpdateMetadata(
      { feePayer: signer.publicKey },
      {
        metadata: metadataAccount,
        updateAuthority: signer.publicKey,
        metadataData: newMetadataData,
        newUpdateAuthority: signer.publicKey,
        primarySaleHappened: curr_metadata.data.primarySaleHappened,
      },
    );
    
    let result = await sendAndConfirmTransaction(conn, updateTx, [signer]);
    return result;
  }
}

process.on('SIGINT', function () {
  gracefulShutdown()
    .then(() => process.exit(0))
});