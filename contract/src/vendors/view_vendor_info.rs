use crate::*;

#[near_bindgen]
impl Contract {
    /// Query for the metadata associated with a vendor
    pub fn get_vendor_metadata(&self, vendor_id: AccountId) -> VendorMetadata {
        self.data_by_vendor.get(&vendor_id).expect("No vendor found").metadata
    }

    /// Paginate through the items for a specific vendor
    pub fn get_items_for_vendor(&self, vendor_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<ExtVendorItem> {
        let vendor_data = self.data_by_vendor.get(&vendor_id).expect("No vendor found");
        let start = u128::from(from_index.unwrap_or(U128(0)));

        vendor_data.item_by_id.iter()
            .skip(start as usize) 
            .take(limit.unwrap_or(50) as usize) 
            .map(|(_, internal_item)| ExtVendorItem {
                name: internal_item.name,
                image: internal_item.image,
                price: internal_item.price,
                in_stock: internal_item.in_stock,
            })
            .collect()
    }

    /// Query for the information for a specific vendor's item
    pub fn get_item_information(&self, vendor_id: AccountId, item_id: u64) -> ExtVendorItem {
        let vendor_data = self.data_by_vendor.get(&vendor_id).expect("No vendor found");
        let item = vendor_data.item_by_id.get(&item_id).expect("No item found");

        ExtVendorItem {
            name: item.name,
            image: item.image,
            price: item.price,
            in_stock: item.in_stock,
        }
    }
}