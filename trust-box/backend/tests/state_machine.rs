use trust_box_backend::domain::TransactionState;

#[test]
fn valid_state_transitions() {
    assert!(TransactionState::Created.can_transition(TransactionState::Funded));
    assert!(TransactionState::Funded.can_transition(TransactionState::InProgress));
    assert!(TransactionState::InProgress.can_transition(TransactionState::Completed));
    assert!(TransactionState::InProgress.can_transition(TransactionState::Disputed));
    assert!(TransactionState::Completed.can_transition(TransactionState::Released));
}

#[test]
fn rejects_invalid_state_transitions() {
    assert!(!TransactionState::Created.can_transition(TransactionState::Released));
    assert!(!TransactionState::Funded.can_transition(TransactionState::Released));
    assert!(!TransactionState::Disputed.can_transition(TransactionState::Released));
}
