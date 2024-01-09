use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::fungible::Mutate;

#[test]
fn it_works_for_create() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let name = *b"htk1";
        Balances::set_balance(&account_id, 100000);
        assert_eq!(KittiesModule::next_kitty_id(), kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id),name));
        // println!("{:?}",System::events());
        assert_eq!(System::events().len(), 7);
        System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id, kitty: KittiesModule::kitties(kitty_id).unwrap() }.into());

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

        crate::NextKittyId::<Test>::set(crate::KittyId::MAX);
        assert_noop!(KittiesModule::create(RuntimeOrigin::signed(account_id),name),
        Error::<Test>::InvalidKittyId);

        assert_eq!(System::events().len(), 7);
        System::assert_last_event(Event::KittyCreated { who: account_id, kitty_id, kitty: KittiesModule::kitties(kitty_id).unwrap() }.into());
    });
}

#[test]
fn it_works_for_breed() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let name = *b"htk1";
        Balances::set_balance(&account_id, 100000);
        assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id,kitty_id,name),Error::<Test>::SameKittyId);
        assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id,kitty_id+1,name),Error::<Test>::InvalidKittyId);

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id),name));
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id),name));

        assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);
        assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id,kitty_id+1,name));

        let breed_kitty_id = 2;
        assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
        assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
        assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));

        assert_eq!(System::events().len(), 11);
        System::assert_has_event(RuntimeEvent::KittiesModule(crate::Event::KittyCreated {
            who: account_id,
            kitty_id,
            kitty: KittiesModule::kitties(kitty_id).unwrap(),
        }));
    });
}

#[test]
fn it_works_for_transfer() {
    new_test_ext().execute_with(|| {
        let kitty_id = 0;
        let account_id = 1;
        let recipient = 2;
        let name = *b"htk1";
        Balances::set_balance(&account_id, 100000);
        Balances::set_balance(&recipient, 100000);

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id),name));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(recipient),recipient,kitty_id),Error::<Test>::NotOwner);
        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id),recipient,kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient),account_id,kitty_id));
        assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

        assert_eq!(System::events().len(), 12);
        System::assert_last_event(Event::KittyTransferred { from: recipient, to: account_id, kitty_id }.into());
    });
}