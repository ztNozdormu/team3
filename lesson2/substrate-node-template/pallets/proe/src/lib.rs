#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch,ensure};
use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
// 主程序 继承了系统特征
/// The pallet's configuration trait. 自定义关联类型
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items. 自定义存储单元
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
	 // 自定义实现存储结构类型 双键结构映射的map
	 Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId,T::BlockNumber);
	}
}

// The pallet's events 自定义事件
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
       // 存证创建成功事件
	   ClaimCreated(AccountId, Vec<u8>),
	   // 存证解除成功事件
	   ClaimRevoked(AccountId, Vec<u8>),
       // claim transfer
       ClaimTransfered(AccountId, Vec<u8>),
	   
	}
);

// The pallet's errors  自定义错误
decl_error! {
	pub enum Error for Module<T: Trait> {
	   ProofsAlreadyExist,
	   ProofsIsNotExist,
	   OwnerNotMatch,
	   ClaimDataIsTooLong,
	}
}

// The pallet's dispatchable functions. 自定义功能模块 实现前端交互的可调用函数
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

	   /**
		* 定义创建存证方法
		*  origin 存证创建人
		*  claim  存证数据哈希
		*
		*/
		#[weight=0]
	    pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
           // 校验操作人
		   let sender = ensure_signed(origin)?;
		   // 判断存证hash是否已经存在 如果已经存在则说明已经有人与该文件做了关联，报出自定义错误（链上已经存在该存证）
		   ensure!(!Proofs::<T>::contains_key(&claim),Error::<T>::ProofsAlreadyExist);
		   // validate the claim lenth is not than 10
             ensure!(claim.len()<10,Error::<T>::ClaimDataIsTooLong);
           // 不存在则将该存证hash存入到自定义存储数据结构中
		   Proofs::<T>::insert(&claim,(sender.clone(),system::Module::<T>::block_number()));
		   // 链上存储成功发送事件通知验证节点进行校验确认
		   Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
		   // 返回结果
		   Ok(())

	    }

	   /**
	    *  定义解除存证方法
		*  origin 存证解除人
		*  claim  需要解除的存证数据哈希
		*/
		#[weight=0]
        pub fn revoke_claim(origin,claim: Vec<u8>) -> dispatch::DispatchResult {
           // 校验操作人是否为签名用户
		   let revoker = ensure_signed(origin)?;
		   // 判断该存证是否存在 如果不存在则报该存证不存在错误
		   ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ProofsIsNotExist);
		   // 如果存证存在则取出对应的存证关联用户和存证区块高度
		   let (owner,_block_number) = Proofs::<T>::get(&claim);
		   // 判断当前解除操作者是否是存证直接关联人 报错关联人不匹配
		   ensure!(owner==revoker,Error::<T>::OwnerNotMatch);
		   // 判断区块高度是否匹配 报错区块高度不匹配  界面新增传递参数 TODO
		   // ensure!(blockHeight==bolckNumber,Error::<T>::BlockHeightNotMatch);
		   // 从存储中移除该存证
		   Proofs::<T>::remove(&claim);
		   // 发送事件通知验证节点校验确认
		   Self::deposit_event(RawEvent::ClaimRevoked(revoker,claim));
		   // 返回结果
		   Ok(())
        }
        /**
           *  定义转移存证函数
           *  origin 存证转移方
           *  dest claim accept
           *                                                   
           */
          #[weight = 0]
          pub fn transfer_claim(origin,claim: Vec<u8>,dest: <T::Lookup as StaticLookup>::Source) -> dispatch::DispatchResult {
                 // 校验存证转移人是否为签名用户
                 let transfer = ensure_signed(origin)?;                                                                               
                 // 判断该存证是否存在 如果不存在则报该存证不存在错误
                 ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ProofsIsNotExist);
                // 如果存证存在则取出对应的存证关联用户和存证区块高度
                let (owner,_block_number) = Proofs::<T>::get(&claim);
                // 断当前解除操作者是否是存证直接关联人 报错关联人不匹配
                 ensure!(owner==transfer,Error::<T>::OwnerNotMatch); 
                 let accepter = T::Lookup::lookup(dest)?;
                 // update claim
                 Proofs::<T>::insert(&claim,(accepter,system::Module::<T>::block_number()));                                                       
                 // 发送事件通知验证节点校验确认                                                                                       
                 Self::deposit_event(RawEvent::ClaimTransfered(transfer,claim)); 
                 // return result
                 Ok(())
          }
	}
}
