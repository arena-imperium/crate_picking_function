use crate::*;

pub struct ContainerParams {
    pub program_id: Pubkey,
    pub user: Pubkey,
    pub realm_pda: Pubkey,
    pub user_account_pda: Pubkey,
    pub spaceship_pda: Pubkey,
    pub crate_type: u8,
}

impl ContainerParams {
    pub fn decode(container_params: &Vec<u8>) -> std::result::Result<Self, SwitchboardClientError> {
        let params = String::from_utf8(container_params.clone()).unwrap();

        let mut program_id: Pubkey = Pubkey::default();
        let mut user: Pubkey = Pubkey::default();
        let mut realm_pda: Pubkey = Pubkey::default();
        let mut user_account_pda: Pubkey = Pubkey::default();
        let mut spaceship_pda: Pubkey = Pubkey::default();
        let mut crate_type: u8 = 0;

        for env_pair in params.split(',') {
            let pair: Vec<&str> = env_pair.splitn(2, '=').collect();
            if pair.len() == 2 {
                match pair[0] {
                    "PID" => program_id = Pubkey::from_str(pair[1]).unwrap(),
                    "USER" => user = Pubkey::from_str(pair[1]).unwrap(),
                    "REALM_PDA" => realm_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "USER_ACCOUNT_PDA" => user_account_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "SPACESHIP_PDA" => spaceship_pda = Pubkey::from_str(pair[1]).unwrap(),
                    "CRATE_TYPE" => crate_type = pair[1].parse::<u8>().unwrap(),
                    _ => {}
                }
            }
        }

        if program_id == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "PID cannot be undefined".to_string(),
            ));
        }
        if user == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "USER cannot be undefined".to_string(),
            ));
        }
        if realm_pda == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "REALM_PDA cannot be undefined".to_string(),
            ));
        }
        if user_account_pda == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "USER_ACCOUNT_PDA cannot be undefined".to_string(),
            ));
        }
        if spaceship_pda == Pubkey::default() {
            return Err(SwitchboardClientError::CustomMessage(
                "SPACESHIP_PDA cannot be undefined".to_string(),
            ));
        }

        Ok(Self {
            program_id,
            user,
            realm_pda,
            user_account_pda,
            spaceship_pda,
            crate_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_params_decode() {
        let request_params_string = format!(
            "PID={},USER={},REALM_PDA={},USER_ACCOUNT_PDA={},SPACESHIP_PDA={},CRATE_TYPE={}",
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            anchor_spl::token::ID,
            0
        );
        let request_params_bytes = request_params_string.into_bytes();

        let params = ContainerParams::decode(&request_params_bytes).unwrap();

        assert_eq!(params.program_id, anchor_spl::token::ID);
        assert_eq!(params.user, anchor_spl::token::ID);
        assert_eq!(params.realm_pda, anchor_spl::token::ID);
        assert_eq!(params.user_account_pda, anchor_spl::token::ID);
        assert_eq!(params.spaceship_pda, anchor_spl::token::ID);
        assert_eq!(params.crate_type, 0);
    }
}
