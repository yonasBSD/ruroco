use std::fs;
use std::path::PathBuf;

use ruroco::client::send;

#[cfg(test)]
mod tests {
    use std::fs::File;

    use rand::distributions::{Alphanumeric, DistString};

    use ruroco::client::gen;

    use super::*;

    fn gen_file_name(suffix: &str) -> String {
        let rand_str = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
        return format!("{rand_str}{suffix}");
    }

    #[test]
    fn test_send_no_such_file() {
        let pem_file_name = gen_file_name(".pem");
        let pem_path = PathBuf::from(&pem_file_name);
        let result = send(&pem_path, &"127.0.0.1:1234".to_string(), &"default".to_string());

        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Could not load \"{pem_file_name}\": No such file or directory (os error 2)")
        );
    }

    #[test]
    fn test_send_invalid_pem() {
        let pem_file_name = gen_file_name(".pem");
        File::create(&pem_file_name).unwrap();

        let pem_path = PathBuf::from(&pem_file_name);
        let result = send(&pem_path, &"127.0.0.1:1234".to_string(), &"default".to_string());

        let _ = fs::remove_file(&pem_file_name);

        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Could not load \"{pem_file_name}\": error:1E08010C:DECODER \
                routines:OSSL_DECODER_from_bio:unsupported:crypto/encode_decode/decoder_lib.c:102:\
                No supported data to decode. Input type: PEM"
            )
        );
    }

    #[test]
    fn test_send_invalid_port_value() {
        let private_file = gen_file_name(".pem");
        let public_file = gen_file_name(".pem");

        let private_pem_path = PathBuf::from(&private_file);
        let public_pem_path = PathBuf::from(&public_file);
        gen(&private_pem_path, &public_pem_path, 1024).unwrap();

        let address = "127.0.0.1:asd".to_string();
        let result = send(&private_pem_path, &address, &"default".to_string());

        let _ = fs::remove_file(&private_file);
        let _ = fs::remove_file(&public_file);

        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Could not connect/send data to \"{address}\": invalid port value")
        );
    }

    #[test]
    fn test_send_unknown_service() {
        let private_file = gen_file_name(".pem");
        let public_file = gen_file_name(".pem");

        let private_pem_path = PathBuf::from(&private_file);
        let public_pem_path = PathBuf::from(&public_file);
        gen(&private_pem_path, &public_pem_path, 1024).unwrap();

        let address = "999.999.999.999:9999".to_string();
        let result = send(&private_pem_path, &address, &"default".to_string());

        let _ = fs::remove_file(&private_file);
        let _ = fs::remove_file(&public_file);

        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Could not connect/send data to \"{address}\": \
                failed to lookup address information: Name or service not known"
            )
        );
    }

    #[test]
    fn test_send() {
        let private_file = gen_file_name(".pem");
        let public_file = gen_file_name(".pem");

        let private_pem_path = PathBuf::from(&private_file);
        let public_pem_path = PathBuf::from(&public_file);
        gen(&private_pem_path, &public_pem_path, 1024).unwrap();

        let result = send(&private_pem_path, &"127.0.0.1:1234".to_string(), &"default".to_string());

        let _ = fs::remove_file(&private_file);
        let _ = fs::remove_file(&public_file);

        assert!(result.is_ok());
    }
}
