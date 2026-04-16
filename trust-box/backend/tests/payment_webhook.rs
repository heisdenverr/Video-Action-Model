use trust_box_backend::services::verify_paystack_signature;

#[test]
fn verifies_webhook_signature() {
    let secret = "whsec_demo";
    let payload = r#"{"event":"charge.success"}"#;

    let mut mac = hmac::Hmac::<sha2::Sha512>::new_from_slice(secret.as_bytes()).unwrap();
    use hmac::Mac;
    mac.update(payload.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    assert!(verify_paystack_signature(secret, payload, &signature));
}
