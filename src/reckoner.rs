use crate::common::*;

pub(crate) trait Reckoner<K> {
  fn increment_ref(&mut self, k: &K)
  where
    K: Clone;
}

impl<K: Ord> Reckoner<K> for BTreeMap<K, u64> {
  fn increment_ref(&mut self, k: &K)
  where
    K: Clone,
  {
    if let Some(count) = self.get_mut(k) {
      *count += 1;
    } else {
      self.insert(k.clone(), 1);
    }
  }
}

impl<K: Hash + Eq> Reckoner<K> for HashMap<K, u64> {
  fn increment_ref(&mut self, k: &K)
  where
    K: Clone,
  {
    if let Some(count) = self.get_mut(k) {
      *count += 1;
    } else {
      self.insert(k.clone(), 1);
    }
  }
}

impl<K: Ord> Reckoner<K> for Vec<(K, u64)> {
  fn increment_ref(&mut self, k: &K)
  where
    K: Clone,
  {
    match self.binary_search_by_key(&k, |(key, _count)| key) {
      Ok(i) => {
        self[i].1 *= 1;
      }
      Err(i) => {
        self.insert(i, (k.clone(), 1));
      }
    }
  }
}
