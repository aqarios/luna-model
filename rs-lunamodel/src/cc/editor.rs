use std::ops::Deref;

use crate::{Constraint, ConstraintCollection, Result};

/// Editable snapshot returned by [`ConstraintCollection::iter_mut`].
#[must_use = "mutations are committed when the editor is dropped; call commit() to observe errors"]
pub struct ConstraintCollectionEditor<'a> {
    pub(super) target: &'a mut ConstraintCollection,
    pub(super) items: Vec<(String, Constraint)>,
    pub(super) committed: bool,
}

impl ConstraintCollectionEditor<'_> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Constraint)> {
        self.items.iter_mut().map(cc_item_mut)
    }

    pub fn commit(mut self) -> Result<()> {
        self.commit_inner()?;
        self.committed = true;
        Ok(())
    }

    fn commit_inner(&mut self) -> Result<()> {
        for (name, constraint) in &self.items {
            self.target.set_constraint(name, constraint)?;
        }
        Ok(())
    }
}

impl Deref for ConstraintCollectionEditor<'_> {
    type Target = [(String, Constraint)];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'editor> IntoIterator for &'editor mut ConstraintCollectionEditor<'_> {
    type Item = (&'editor String, &'editor mut Constraint);
    type IntoIter = std::iter::Map<
        std::slice::IterMut<'editor, (String, Constraint)>,
        fn(&'editor mut (String, Constraint)) -> (&'editor String, &'editor mut Constraint),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut().map(cc_item_mut)
    }
}

impl Drop for ConstraintCollectionEditor<'_> {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.commit_inner();
        }
    }
}

fn cc_item_mut(item: &mut (String, Constraint)) -> (&String, &mut Constraint) {
    (&item.0, &mut item.1)
}
