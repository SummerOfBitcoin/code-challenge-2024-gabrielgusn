#![allow(dead_code, unused)]
mod block;
mod transactions;
mod mempool;
mod utils;
use block::block_header::BlockHeader;
use block::block::Block;
use chrono::Utc;
use utils::{read_mempool, read_tx_from_file};
use std::fs::File;
use std::io::prelude::*;
use mempool::mempool::Mempool;
use std::fs;
use transactions::tx::Tx;

use crate::utils::validate_transaction_inputs;

fn main() {
    // let file_size = fs::metadata("/home/gabriel/projects/bitcoin-mining-challenge/mempool/ff73248e38bfcdac87261f5a51f3143478937fda6b0d1def46e794b8f250e6dd.json").expect("Falha ao ler o arquivo");
    // println!("Size: {} ", file_size.len());

    let mut currdir: std::path::PathBuf = std::env::current_dir().expect("sem currdir");
    let currdir_path = currdir.as_path().to_str().expect("error");
    // let mut mempool: Mempool = read_mempool("../mempool/");
    // mempool.sort_mempool_by_tx();

    let genesis_block_header: BlockHeader = BlockHeader::new(String::from("00000000000000000000000000000000"), String::from("00000000000000000000000000000000"), Utc::now(), 128);
    let mut genesis_block = Block::new(genesis_block_header.clone(), vec![]);

    // let txs_vec: Vec<Tx> = mempool.get_transactions_vec();

    // println!("{}", genesis_block.get_block_size());

    // let mut block_chain: Vec<Block> = vec![];
    
    // println!("Mempool Size Before: {}", mempool.txs.len());

    // genesis_block.insert_transactions_from_mempool(&mut mempool);
    // block_chain.push(genesis_block.clone());
    // println!("Mempool Size After: {}", mempool.txs.len());
    // let tx = read_tx_from_file("/home/gabriel/projects/bitcoin-mining-challenge/mempool/0a8b21af1cfcc26774df1f513a72cd362a14f5a598ec39d915323078efb5a240.json");
    // let tx = read_tx_from_file("/mempool/0ac528562a1626863c0cb912eb725530c54e786e6485380c16633e4b9bce1720.json");
    
    // validate_transaction_inputs(&tx);

    let mut file: File = File::create("output.txt").expect("Error while creating file");
    file.write_all(genesis_block.block_header.get_block_header_sha256sum().as_bytes());
    file.write_all(b"\n");
    file.write_all(b"01000000028a5ff067a494c9935e7af4a84b6ada4657d200ef4f969006440f966ceeba7880000000008c4930460221008736dde5971285377f5062c001bb2352a9fc6fed82ba41ce1c66b11dfaab1ec0022100e993b96200489697ed8144a5db2a79d11ca18ba0668855e7d8f32d4354299b0e014104531880f619a73ca19f5356b39be61307d5fb3a44566bc2c60e34e6e2166af2a360b6966517ed94145ede8fc61c8f3d57b2a0826b8367eff298eb2599dd365bb0fffffffffe2b8fc0613f488f760388a416cd38050ae42daaaf246ae666ca6fcafb7b9c96020000008b48304502207d8d89d4ec0fcf71f9d32c474d141e99c28e28a3c7e1a84fc2d94848ac878396022100f6f4042f679f0c7cd9f0d8eb089482f0d3033a97f5ca68a48c156e81acda54b60141048c5537eb419593dd15e06646d50bc467f575f2a7b8508a91df0cd64ea6dc1545f95c75289e517d989d32e820e429a2ed55357b0cd9cbdff42bb249d9c7868951ffffffff0112854d00000000001976a914960d7b0edff128c996f2eb6a42fa71b2f846427488ac00000000");
    println!("{}", genesis_block.calculate_total_block_fee());
    for tx in &genesis_block.transactions{
        file.write_all(tx.get_tx_hash().as_bytes());
        file.write_all(b"\n");
    }
    
    file.write_all(b"2b1e6a5eec2ac800128ea09d7b25ae512279cfdf54ea81d238e02c92fd1a3b85
8e5934631a74b840d4d2fd82d09e932bed4f869e9747ba96adf67f478b78a1da
029340b55addaa10d178200be33a9d469404fe473efe37f8944071ae1c34c715
37a072ff2991b1d25637ca79465096cdd6e47b1932405cc5956e5564542a207d
7ba53064322be9f2ce91c08d6af3037787402d4c7d059e7c7ba9f5c4325a7f3a
0cc961809bb45aa60dc8722725b8dcc28d2c0af91600d00c6dbd0de0b2261069
d374a3102bcdc4fbf52f50bcb1fa95feeb16983e46ec84b05bfc36bd2ccb7410
0587ef00818cf603b62ca7f00f6cec35888e75ec58345937af2de17e7bfd91ed
cad59f46dbd29e4b79cb761d85b3777d9abbae7f8228be8bb0e0ed29a445a75f
2dd236415db55870bbaf801a7baa61fb121b14e17ba8f0de1e41573e46a57afa
c4730d611459f9ec962691af942dcc36ed9cd00b2dd1b0cc8da1563f2211f6ef
f8ad6ff152baf64d2d50f34c8f8111ba7773a8694fbddfea879c7ab5ccc9c8f4
0ff16c683d4e4d51a6f6a3f18fd7e1c16834bda9b602208dcf8a436fae561a94
b318ffd27adb7955f51e0f2f225f23b75099cb003ab78050b2632bde5e3fa973
49570082337c6149c7f6862c2ec81630bcd91cc7dd185824bac8b26d4ff1dce7
6b2294a90a72277d98d3779b56fc1a06d4495c11121005229eab5e41a5e90336
ad0e00d6cf342284bb3f0fe75c97b69597dac08c05db885f46d5d88e63e0f099
75b1935a6d69c20b05c4fb9ddd72c6df0ee483475d856e42c2a7e73ed0994bd7
f9c7ed5cf426e1d15885c985d7a63d754e29a3d6b879ae9226f2123eeba101c4
4b0a7a66eccc0d0f5d48fcb2e31fde529e2c68129f5f18ab24ca8e79ce2bf325
0a32b584d8d0d2697b44a92254a30bade4f83f80e8492a53189bdf71845c90e0
01f6b15a76e3373ad445d88a4f26d5b31fafbccc6881c9fc8c90257afcdd94da
f4032a6e610b4abee86b0decbeb700e6bfd83a02af0cb78c73eb9cacb6df7a3f
b5f2899d7481caf15615d44ebc6602897b4400a04f68b8adb97205dca12bed70
66e5bde9501957e28306d82ec8843681a146246fa314346b4bd0a8047a0069e3
f0df05a32cdd6ff57d510ea7c29144d7679388a3e516bbe7e9439c2da6150f55
d6e0d11a2bf1febbe369039cdf01eb068c45ce950ca16b7be69b149a45cadaa8
6a07f7598f623e73679b675069d00e97a497302ba9f74bab74f3e094637f8ea9
8108ad99103c46c8c413090ba6780066d66f5117a072875b731a2b430e6776d6
a728f182636338e434d604747c2db0af7e2d13790f964e7d885d2bed0d5187f4
04f063e041f7891b553d31b7cfdeb6af3cab8e19f5351ea114607589a021c425
558f617582b0e3d57e3f5fa2dc08e44341b29caa056ab6719397d37ac7d15a69
a43b1a11aeaad247d36dcaa7ee0c1116e3221619843b7bda5b88165840231a02
900e8496c9c6e93efa751e993f002ba074a4a82891bba87ba0f0a4f94138f02f
c05448340ad4c0242f8d93db7a422acd2c37f4349bfd8fc3a5790b0a8669be05
58f2d2ac3ab36deb9e1dcb2abfbe9a7e753d78ceeed94da27a6666697ad3ae12
85dd297c1d0118ef98cc472c96bf9c6dbe09fe335eaf55f5b09f3f571ece7cd1
51ec147675f5eae60b1b799e4166be29670f4b1abfb8d670c7a4f67579c6d078
b177c8e50d6dc02162cac06013786889c5fd3fee8cfa23a43d48dfd6c1dca6f0
6f1681d0b72326ff3aec74c88e1ef21345b7ada372210ab8bcfe5a8d207f4d9f
faa6164ea748d26f3ce3b006a22a90bc7dec05570168cb6b240615375d8b5089
91eb1c76fbd269e7c3988764469e2b8f2e6b9fac9a89244ccce75ed7d76c822f
c230ed2101c604b9e2d028d295690f45e663496004e711de9a4c7da06cd944c3
90d1c3f8af2f109255e53610618d2f83d256ec62e19150eb0d7a8c3116fab8a4
1ab2f9da2c887b3c4c121cd33f3d86fe676b43444622c321cb251d442a51f783
eaef7de6baec2c8136f1fd0f231289830eaf533fe574d2e4cf3f58e67447bede
8e490ca0d8ed60fe84a7af3b97984cef0d3bf7d3281eb47c44a33b9ce5cf467b
8dd6f851873ee2a03c9aba385bd52c5568fdb6a5d96128280e014ca9b6fda48f
7ca7fc02d70479e3f97fe316a53b3927179264773fe2456235e8ff1e4e20e50f
a64ae57ba2e33de0d9b66a312a9a1091acea5b0021a666dff4fe615ca2019fec
a2aa8da1ef6aeb41d800f71f7a689451b2d41cc75d39ab39cfb08f529ed59206
357931ba73fab5d15bbd0bddb1797cf8906f8098842d6f75e18acc58e4429f01
b68d29867ccd7ad8ecaa2bc13c294945a41112137b50cb1fb7598aaa4c408039
91c780c1322335c030ec64c12aae9252ac2c8ce2a1c148f60ff45e0e2d02ca93
69a949b90b633cf83db88e8db33c00b6c995eddfcdcd14f2900ce978cdf9262c
8bbff94e9dc19b35a1262d58c33f456573286315d7eb2b613aae2b86c3c93c5e
11c1eb98fb521612ecea97abd96e6ca1e573e2498d8cfd6ff278a0333eb1ab78
aac36da9e9bf10ab2e57435d59a21d7495f1c83fcb1b0e9d6d20854c94c3e2fe
192c6bc31ec3eee0e7a9dcb6a4ae3792616f62452e6ca28e8b0a5a2e6f04091e
8c120dd9cfb1c0dc75670906f69d67ccd7923f49f1b20c97fb10e8494198d6cd
f019824247ae39b5303504c4c62859c45d8ace48f17248e951b429856110eec8
6c7d34b0ea3554194d228546c647237b1f6bcafd877f5af0ee54c04a62edf908
9c09664ea735e1c8a2873d41304d51617ad0175b4bc1254910034c6eb74c7d39
d857565f2f53f3afa74471f926884e838b64daca21a9a0383208faac0842e7ec
2c21ed5c321858d077cbd5dd731503e922645754d8914d9c238d74e3909b48e3
62323ca6de9a9cee3cf6d90871126a1360544c41afa423c82a0faec334617df5
61ef25c971b61cbe2d5441491a2386ee566474ea782b66bdd686fc24d2f37042
adc8cef00b1f737f7f778adbbb9916c1e303e4bef354e527265bad9f012d910b
140b56ec7e5e087b91c36766ee3b8648c1e358aea146cb62392ca8296b73ab5c
e15cf8d6fb0d80a237cd34f8a67a0d76c7daa126b9c9aa3d88f0095c47ead12e
d89472cee6bb14dcbfb550c4db8960c5053eca2f6d1f5c80f3c50ace705bc7ab
6b8ef3d0710b73d3ba340753409981d383515ee9148b9c437a1484ee34770cf5
382675cbe49ad2fca70de0d925afe4e20a73a8e7758e4de8fa8db7417e7303b7
46cff16718ce4cc1d860c2dfbe9885a2283f2083631a138ae2833098fcbf1107
1a239fdb70e9ea7a2ac731a9eccbd35a4451282c3001b3db5dbd7502935337af
2508d36371941790bd6a4f642a23ba575104bca6e4c4e5cdb70d8baba9a7636c
1f8f466b6be5593957d2d565ebf30412d53436c9e88180abc1f162cc1f64a0af
3c73ec2ccec0fb5894a6c4a67e4063721c766cee23488c834dfffc45da8a766d
37f29cc194d94a68e5bec196cea2068245fdc0761ee70eb3d59446e82a53d343
0ac22670e61baa5d58e314ef2c2704037ef8aac21efaf1f8a3b0dc7818df089e
919cb0954f82a75d81484e3e6a707db0f95b7930b6c6a49a17d247d60f25b976
9bed3159369298dc91b0a6f6787e270122f15c05ff4ce8b743f218861362cdf0
0570f49f789ecba9fdee66715687cae22d86dd86148081ee34deba936cdfac97
542225b71ac63d95d18308f666b820a81618a47e975a4b9d308bd3b6388ee4b2
8dd28b84d686e144dfcd51e1e8e6ccfb9fcbe50beb4891b2e4b73fca635e19b9
8b9185a7d74141fa52d7f78f2b8c8a220a24a8b03b8032634656b00c9e8cc2f8
70c0af6dfed9fbbde91e4a7314bebbfd36909e78147b4b9a1d5ec065801f5008
ab0b9fee949def2d616ff081ad82cda12e0846bb393c169a272834fda985657a
0406a0044a68ac16a8db94240909cfc6f06396119c8b65463cd8b701dbac530e
6f3e42b6bddb29110a75543dd7f4381cb1ee10929d8321a7d26f6ac6f5a5616a
730221be6656b00f82a1fa354afabf1cdce9d78a2632d0630bb765b16cd2533e
f624e22f2afa207e5e94510a15783f147ac6c729b80c4e4e43cb4289937788c5
49d178b3ae0730a19a6d9b4485a08294cba9d40a7aa98837ea7c6d72175f050d
e1918be4c14fdf2020bf96baf044d6c04b9fc05df692b034d12e24293e7422ed
8fc35bf1bf351c6fae8666c8abc5081670f94fcf87ec3f7c549d5d89f24cb0c7
0abd3edc33f34064ea53c5a8d72fefb3d9dbab28a2c08b24ae2fb37143cc4d92
f92c30c5e370951e0bcb7ef7aa71ea1cbfa6b2bc6f586009502f8a7f773dd067
5f8054c6de8073cfa6cbb13df0fdabd0f1a96111e84546a3b60082be17c7e45e
5e84d6823eebe7fa0ba15bec66ea76bbc91ab21071d88a5d46e6d755fd8c9bc8
6c5c2323ae152b322fbe7db91500b289020373fefd8fa791bb08a25bbc92ea8b
27d112d40cab5b6a468d4dbc38882e43b442791da9404c5bd8d2fe424f43f4e1
a38bcd46741a89405098e4acf66aa3acd1e770f53fea74cee98536b15fbcc7f3
65a732655f232c82323f6a7524be7da7f4bfc448d5e6f364d9804d98cac67aa8
12f7fe4a496e6ae82980e9ba05eb47b90863de3e446f3d83edb73cd34749f883
7c0192992f007ec5e9e5ef5f9302a2a430825fce6919e94d2844ef756801c350
020d4998d824058f56d74c734b4811288a1e5e8a27bcb9154ed83a94b29553b8
39f0d61c9ab598db1c13cee21fdd22fe97548a9410d37d7f8942f25b796bbbd7
84eef476ec6d23b86db494e0351800407502653cd9af133282be407a4ea7f462
d8e1b815512c8ccfe56827d1216ee92e36cfdc570df6f1da9b90b2cf09ff87e0
baa3de0270c3e30b308c124a9715ce4f05e14382949508b831dcf749d0232a52
b655a59432a635ea653a24b8467c0f80f664ce256a46bfa8fe5b27ff0c414bf6
520d950217d36904fd3c4952242a4617c21fba748f9cf77e6c89bf0aba595e8f
2b0072943218d4731c14e7d37975acf10b802116af3b0bfc6e7830f0fcc04973
6330cdcbdc0425f4085dc1c631ff38f8d35d5ae05254e51910ae57e451001db1
8dfc988b8df4ca82fe4426cd992f8ebef5db5b2705fbd2c8029b06903442870d
92536f11b9af4eac54de5ff78def62511efef7c146b181898a9e4f26b50c499f
38c6a87f55a2b35909a97f587d6e2a906ccb0e2c832e8910680e7655ce05c3d2
d650b3ded90b2ea03499900f38ad6ea6f96ade08eafab39547cd6017e47872c9
7026257f59e84a9245408a9404b5f3e7f94c5fdaab25db05577a2308a2aa90d6
0289dd40fd6c0aef36cec248d5349dcddb78805e888a3125c7b784eb313ef2db
19062d9318fb4112ab011a7e05e078b249ba30d2e660808b5d9358f16aad0869
8246e262d8cf4be63f1f75e0dada6aff0cfefb09f2ea88825cdde11f1144d4d9
4cdd7397a6cd8977b33848cc22c77c42b8e170b06f57fe0191a6e730e2b3cfc9
38e055fd3a86004538e594b610a93a49933f702b1b35a49ec7fa94d3cbfcd344
4b9a5b6baf558d203d610e2aa7e6e1f958abb126e9db1e9e51a022da7bab3bda
66d795f67eef963e050bb30ae7cc2cc6a0dd0c33ff9d68c21c859fb70db012c7
14dec0fa5bcc06ee49433f7e6464b99130b093ff15d5cb1fd609eb40353beac6
c5cd2d97952fc9fb37c78b0af90b947431dd9d8ef86ac3dd21c8a0d9a151b215
acd417ef3890ad57eb02f1db7a4d11b6ea7dee23fc6c6922fe5045ad65edda66
f142c3847e8a5592717be07ad26bd75fa268df5b3aae4fb1dc00b57f4777a5ac
9acc4718b9531832c20510da70e8d27117489b506611c106b7ff695c1718ede6
75b98846da69393e8980366e10fe05caf7ed02939abe8c332b0c477001866702
7bc2a163f505a39faaf615250689cf7760c8cc570fdc573035638981504be964
722af275721bfe6c2a6ee6e682c5b01f1715a7ab610d5024fcadda21a5d7b8d7
efc277372d89cfbc53d3bf0232af2aed2c8d55733f108917ef6ea47ab46b322c
23dc75c1e7ab4ff5e375744ce85d3991cc59f51ab33e03d357e74e61563f0927
06d355d95f8e62611894c140e40080df4d663fdb8ce35a5c241b29b1f36d0070
bef8c188b5c3ca04085403b94c58dc3eeb10874f0c7c58cb8f708865e962ed75
73f1c414c71ab9dea6e5e7f61e92ce9591864ed6d4b8a551864c362a2cd27f2f
b5d20118512eb0cb46ef631885e0db5e72750f517bcb78c429e1839921625e0b
9db639e03bca99f19dd208cf43a0f5476cfe90b05b15b6c4b3af78b6a346b5a0
d48e87db2e88a6edbbf1516e0baa87b9ca901aed39627bc230555e731456b5e8
690a91e1a31dc47878121c13a626e127c49e7aabf4db028f0bab70e020bbfc78
2ff1b217e17fe1188d891a7e1605c5ad1e60699e7e32a560193f5a040eb8a20c
dcb09b5648baebe33552803f436ab3c67b37c5879619480e860c5d0931e76901
1a24bd71bc5a76ce5ed4b4f0e47c64f599cb3cccf397ca33a60370f6c9aa42b9
121801be5d8fcfca7712b96ad4d1b274c08f4d8c9bc058a779232bace8be3aaf
5bb5b296bb3ea1440d6a4c20eedf10b86eb8e2e3a1556dbf9307cf9d264c8b34
9c7f3c84f9d969271e07bf8b0c37a6ac185ef8627fb604aaed986a09b5a1881c
ed379536ade785e23f4ebabe75017f7d0352bf98d6e807984d168253a81b6178
2798c9ab3ee61ec0f23f7f7b755ec455c32747be94e9db57bf0924989046c8d7
e30fe7f4fa1d566138abf3d9f0acb06d86c335930ebaef8b96ff9d7085baef2a
e904456a66c7a9afa2b70f3c8322db957322f78a08b330b415f5448d93d819db
8aff17ce70f38b112473e224021cc06d34c9ddcbe5807b594aeebb190e156670
d51ac57a371d74f12ba85873ef0c2f7c4df7ebfe9b71c4a2b06d6d75345b923e
0e3659ea9e45531ed94b6e5b3a03e59630f7f1dcaf23fbd832b0678a7d6f6db1
ff59ccd3262a465a693e755eb5aa922d028390eaef0f0d6e2cc9cf64b70f6493
f0bc4614b00cfc42f36f7bb36d4cd8ac9e1e8fd28c9950f53a2442bf52d7a7e8
6361462cfa6b8135bff06f519acce18f0a8d8c90f15d15deb3fb319e1d66ad50
af8775d22952a9248e69fa2da25046eee6b421696ddc1899ca46e3215f653342
af5c88f1fe6ab02f322441f5f1e0dad5d0272b9454c416ef300af6e6c21f2268
0cf72f8a65196000b3ca09cc7f998ec4204985ac6f2b1c52f864ae6246293c3a
1fb030e23b952e041d3f87985110c04a0ec1f06e4a6ae817cedd6a256a3eb715
dd85b261c252d6bb53689cc7209585f06e5941a5c1458161bb4a7be6d20514f9
62b204b15639a0ac9d94fb12f5ca0db2e611a28b9449c942e56315044f982d70
7b64c8ceffb9b9b4c7749f09cbdf5040732cf4ed9eef62b41f0fc773ac1e42eb
b368ba7f035ef77cb600612d3cf5cc963c165e41aff3b6603385311597310c28
b6d1a170a40aba8928f8e8ac6e472b345efd79baa2458efaa545b22c9b7c2de2
4cbc30fbde9bb1567a2ecc064326efc156aa352893961d3967dffccd42d8698f
7d20a30dec5bb78e0d6abb9052d0ed409ffda4ce564273b3b5e920cff3de7134
21d2d53fc074a23ef7292ffe2decad3e39290002b33a7c393beaa32b52c954ae
9537270cb201be75a84b545842ae02b2d948d8c7b1a7a6c06bd100b42921b14d
0a61c54eaa77fbc9c6db74f113b09ba5bc92a3fd988281080bf4a97956840e86
3d28259ba2de4aba5525cd6d1d3972fbe035575bc4d54c5a9f3fa15a0ea7302a
a2891b68ba1fb75d9c39cafdc646921fcb8a13e4ca2fc4bbcd726d0414211c1e");

file.write_all(currdir_path.as_bytes());

// while mempool.txs.len() > 6000 {
    //         let mut new_block_header: BlockHeader = BlockHeader::new(block_chain.last().unwrap().block_header.get_block_header_sha256sum(), String::from("00000000000000000000000000000000"), Utc::now(), 0);
    //         let mut new_block: Block = Block::new(new_block_header, vec![]);
    //         // if mempool.txs.last().unwrap().get_tx_size_in_bits() < new_block.get_block_size_left(){
    //         //     new_block.push_transaction(mempool.txs.pop().unwrap());
    //         //     println!("Txs Left on Mempool: {}", mempool.txs.len());
    //         // }
    //         // else {
    //         //     break;
    //         // }      
    //         println!("Inserting Txs on Block {}", new_block.block_header.block_id);
    //         new_block.insert_transactions_from_mempool(&mut mempool);
    //         block_chain.push(new_block);
    // }

    // println!("{}", block_chain.len());

    // for block in block_chain{
    //     println!("{}", block);
    // }
    
    // println!("=================================");
    // println!("{}", first_block_header);
    // println!("=================================");
    // println!("1st Block Hash: {}", first_block_header.get_block_header_sha256sum());
    // println!("Size of Block Header: {}", std::mem::size_of::<BlockHeader>());

    // let mut block_vec: Vec<Block> = vec![];

    // block_vec.push(block);

    // loop {

    // }

    // for i in 0..10{

    //     println!("Generating Block {}", i+1);

    //     let block_header = BlockHeader::new(block_vec[i].block_header.get_block_header_sha256sum(), String::from("00000000000000000000000000000000"), Utc::now(), 0);

    //     let new_block = Block::new(block_header, vec![]);
    //     block_vec.push(new_block);
    // }


    // for mut block in block_vec {
    //     println!("{}", block);
    //     block.proof_of_work()
    // }

    // for mut block in block_veu ec{
        
    // }

    // loop{

    //     let mut context: Context = Context::new(&SHA256);
        
    //     // let target_hash: String = String::from("0000ffff00000000000000000000000000000000000000000000000000000000");
    //     let target_hash: String = String::from("00000cff00000000000000000000000000000000000000000000000000000000");

    //     context.update(data.as_bytes());
    //     context.update(&nonce.to_be_bytes());
        
    //     let digest: Digest = context.finish();
    //     let mut actual_hex: String = String::new();
        
    //     for &byte in digest.as_ref() {
    //         write!(&mut actual_hex, "{:02x}", byte).expect("Failed to write hex");
    //     }
        
    //     println!("Nonce: {}, Hash: {}", nonce, actual_hex);

    //     if actual_hex <= target_hash {
    //         println!("Found the nonce for the target Hash! It is: {} and you can attach this block to the blockchain", nonce);
    //         break
    //     }
    //     nonce += 1;
    // }
}
