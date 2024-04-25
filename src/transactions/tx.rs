#![allow(dead_code, unused)]
// use ring::digest;


use core::{hash, fmt};
use sha2::{Digest, Sha256, Sha256VarCore};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tx{
    pub version: u32,
    pub locktime: u32,
    #[serde(rename="vin")]
    pub tx_input: Vec<TxInput>,
    #[serde(rename="vout")]
    pub tx_output: Vec<TxOutput>,
}

impl Tx{
    pub fn new (version: u32, locktime: u32, vin: Vec<TxInput>, vout: Vec<TxOutput>) -> Self {
        let mut tx_size = 32 + 64;

        vout.iter().for_each(|output| tx_size += output.get_tx_output_size_in_bits());
        vin.iter().for_each(|input| tx_size += input.get_tx_input_size_in_bits());


        Self {
            version,
            locktime,
            tx_input: vin,
            tx_output: vout,
        }
    }

    pub fn get_tx_output_value(&self) -> u64 {
        let mut output_value: u64 = 0;

        for tx_out in &self.tx_output{
            output_value += tx_out.value;
        }

        return output_value;
    }

    pub fn get_tx_input_value(&self) -> u64{
        let mut input_value: u64 = 0;

        for tx_in in &self.tx_input{
            input_value += tx_in.prevout.value;
        }

        return input_value;
    }

    pub fn get_tx_fee_per_bit(&self) -> f32 {
        self.get_tx_fee() as f32 / self.get_tx_size_in_bits() as f32
    }

    pub fn get_tx_fee(&self) -> u64 {
        let mut inputs_total: u64 = 0;
        let mut outputs_total: u64 = 0;
        
        for inputs in &self.tx_input{
            inputs_total += inputs.prevout.value;
        }
        for outputs in &self.tx_output{
            outputs_total += outputs.value;
        }

        return inputs_total - outputs_total;
    }

    pub fn get_tx_hash(&self) -> String {
        let serialized_tx = format!("{:?}", self);
        let mut hasher = Sha256::new();

        hasher.update(serialized_tx.as_bytes());

        let result = hasher.finalize();

        let hash_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

        return hash_string;
    }

    pub fn get_tx_size_in_bits(&self) -> u32 {
        // initial value for tx_size because every transaction will have at least 64 bits
        let mut tx_size: u32 = 32 + 32;

        tx_size += self.get_tx_input_vec_size_in_bits() + self.get_tx_output_vec_size_in_bits();

        return tx_size;
    }

    pub fn get_tx_input_vec_size_in_bits(&self) -> u32 {
        let mut input_size: u32 = 0;
        
        for input in &self.tx_input {
            input_size += input.get_tx_input_size_in_bits();
        }
        return input_size;
    }
    pub fn get_tx_output_vec_size_in_bits(&self) -> u32 {
        let mut output_size: u32 = 0;
        
        for output in &self.tx_output {
            output_size += output.get_tx_output_size_in_bits();
        }

        return output_size;
    }

    pub fn pre_validate_tx(&self) -> bool {
        let mut tx_validity: bool = true;

        if self.get_tx_input_value() < self.get_tx_output_value(){
            tx_validity = false
        }

        for output in &self.tx_output{
            if output.scriptpubkey_type == "unknown" {
                tx_validity = false
            }
        }

        return tx_validity;
    }
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxInput{
    pub txid: String,
    pub vout: u32,
    pub prevout: TxPrevOut,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub witness: Option<Vec<String>>,
    pub is_coinbase: bool,
    pub sequence: u64,
}

impl TxInput{
    
    pub fn new(txid: String,vout: u32,prevout: TxPrevOut,scriptsig: String,scriptsig_asm: String,witness: Vec<String>,is_coinbase: bool,sequence: u64,) -> Self{
        Self{
            txid,
            vout,
            prevout,
            scriptsig,
            scriptsig_asm,
            witness: Some(witness),
            is_coinbase,
            sequence,
        }
    }
    
    pub fn get_tx_input_size_in_bits(&self) -> u32{
        let mut tx_input_size: u32 = 32 + 64 + 1;
        
        let txid_size: u32 = self.txid.len() as u32 * 32;
        let prevout_size: u32 = self.prevout.get_prevout_size_in_bytes();
        let scriptsig_size: u32 = self.scriptsig.len() as u32 * 32;
        let scriptsig_asm_size: u32 = self.scriptsig_asm.len() as u32 * 32;
        
        let mut witness_size: u32 = match self.witness.clone() {
            Some(witness_vec) => {
                let mut witness_vec_strings_size: u32 = 0;
                for witness_item in witness_vec {
                    witness_vec_strings_size += witness_item.len() as u32 * 32;
                }
                witness_vec_strings_size
            },
            None => 0
        };
        
        tx_input_size += txid_size + prevout_size + scriptsig_size + scriptsig_asm_size + witness_size;
        
        return tx_input_size;
    }
    
    pub fn is_coinbase(&self) -> bool {
        self.is_coinbase
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxPrevOut{
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

impl TxPrevOut{
    pub fn get_prevout_size_in_bytes(&self) -> u32{
        let mut prevout_size: u32 = 64;

        let scriptpubkey_size: u32 = self.scriptpubkey.len() as u32 * 32;
        let scriptpubkey_asm_size: u32 = self.scriptpubkey_asm.len() as u32 * 32;
        let scriptpubkey_type_size: u32 = self.scriptpubkey_type.len() as u32 * 32;
        let scriptpubkey_address_size: u32 = self.scriptpubkey_address.len() as u32 * 32;

        prevout_size += scriptpubkey_size + scriptpubkey_asm_size + scriptpubkey_type_size + scriptpubkey_address_size;

        return prevout_size;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxOutput {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: Option<String>,
    pub value: u64,
} 

impl TxOutput{

    pub fn new(scriptpubkey: String, scriptpubkey_asm: String, scriptpubkey_type: String, scriptpubkey_address: String, value: u64,) -> Self {
        Self{
            scriptpubkey,
            scriptpubkey_asm,
            scriptpubkey_type,
            scriptpubkey_address: Some(scriptpubkey_address),
            value
        }
    }

    pub fn get_tx_output_size_in_bits(&self) -> u32{
        let mut tx_output_size: u32 = 64;
        let scriptpubkey_size: u32 = self.scriptpubkey.len() as u32 * 32;
        let scriptpubkey_asm_size: u32 = self.scriptpubkey_asm.len() as u32 * 32;
        let scriptpubkey_type_size: u32 = self.scriptpubkey_type.len() as u32 * 32;
        let scriptpubkey_address_size: u32 = match &self.scriptpubkey_address {
            Some(scriptpubkey_address) => scriptpubkey_address.len() as u32 * 32,
            None => 0
        };
        
        tx_output_size += scriptpubkey_size + scriptpubkey_asm_size + scriptpubkey_type_size + scriptpubkey_address_size;
        return tx_output_size;
    }
}

impl fmt::Display for Tx{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "Tx {}\n\tTx Fee: {}\n\tTx Size: {}\n\tTx sat/bit: {}\n\t\t Total Input: {}\n\t\t Total Output: {}\n",
            self.get_tx_hash(),
            self.get_tx_fee(),
            self.get_tx_size_in_bits(),
            self.get_tx_fee_per_bit(),
            self.get_tx_input_value(),
            self.get_tx_output_value()
        )
    }
}