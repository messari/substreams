use parquet::basic::Repetition;

#[derive(Clone)]
pub(in crate::streaming_fast::file_sinks) struct RepetitionAndDefinitionLvls {
    definition_lvl: i16,
    repetition_lvl: i16,
}

impl RepetitionAndDefinitionLvls {
    pub(in crate::streaming_fast::file_sinks) fn new() -> Self {
        RepetitionAndDefinitionLvls {
            definition_lvl: 0,
            repetition_lvl: 0,
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn repeated_item_newly_seen(&self) -> Self {
        RepetitionAndDefinitionLvls {
            definition_lvl: self.definition_lvl + 1,
            repetition_lvl: self.repetition_lvl,
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn repeated_item_previously_seen(&self, max_repetition_lvl: i16) -> Self {
        RepetitionAndDefinitionLvls {
            definition_lvl: self.definition_lvl + 1,
            repetition_lvl: max_repetition_lvl,
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn get_definition_lvl(&self) -> i16 {
        self.definition_lvl
    }

    pub(in crate::streaming_fast::file_sinks) fn optional_item_seen(&mut self) {
        self.definition_lvl += 1;
    }
}

pub(in crate::streaming_fast::file_sinks) struct RepetitionAndDefinitionLvlStore {
    definition_lvls: Vec<i16>,
    repetition_lvls: Vec<i16>,
    max_repetition_lvl: i16
}

impl RepetitionAndDefinitionLvlStore {
    pub(in crate::streaming_fast::file_sinks) fn add_lvls(&mut self, lvls: RepetitionAndDefinitionLvls) {
        self.definition_lvls.push(lvls.definition_lvl);
        self.repetition_lvls.push(lvls.repetition_lvl);
    }

    pub(in crate::streaming_fast::file_sinks) fn add_lvls_for_optional_field(&mut self, lvls: RepetitionAndDefinitionLvls) {
        self.definition_lvls.push(lvls.definition_lvl + 1);
        self.repetition_lvls.push(lvls.repetition_lvl);
    }

    pub(in crate::streaming_fast::file_sinks) fn add_lvls_for_packed_field(&mut self, values_read: usize, mut lvls: RepetitionAndDefinitionLvls) {
        lvls = lvls.repeated_item_newly_seen();
        if values_read > 1 {
            let definition_lvl = lvls.get_definition_lvl();
            self.add_lvls(lvls);
            let num_lvls_to_push = values_read-1;
            self.definition_lvls.extend(vec![definition_lvl; num_lvls_to_push]);
            self.repetition_lvls.extend(vec![self.max_repetition_lvl; num_lvls_to_push]);
        } else {
            self.add_lvls(lvls);
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn get_definition_lvls(&self) -> Option<&[i16]> {
        Some(self.definition_lvls.as_slice())
    }

    pub(in crate::streaming_fast::file_sinks) fn get_repetition_lvls(&self) -> Option<&[i16]> {
        Some(self.repetition_lvls.as_slice())
    }
}

pub(in crate::streaming_fast::file_sinks) struct RepetitionAndDefinitionLvlStoreBuilder {
    max_definition_lvl: i16,
    max_repetition_lvl: i16,
}

impl RepetitionAndDefinitionLvlStoreBuilder {
    pub(in crate::streaming_fast::file_sinks) fn new() -> Self {
        RepetitionAndDefinitionLvlStoreBuilder {
            max_definition_lvl: 0,
            max_repetition_lvl: 0,
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn modify_lvls_for_struct_repetition(&mut self, repetition: &Repetition) {
        match repetition {
            Repetition::REQUIRED => {}
            Repetition::OPTIONAL => {
                self.max_definition_lvl += 1;
            }
            Repetition::REPEATED => {
                self.max_definition_lvl += 1;
                self.max_repetition_lvl += 1;
            }
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn revert_lvls_for_struct_repetition(&mut self, repetition: &Repetition) {
        match repetition {
            Repetition::REQUIRED => {}
            Repetition::OPTIONAL => {
                self.max_definition_lvl -= 1;
            }
            Repetition::REPEATED => {
                self.max_definition_lvl -= 1;
                self.max_repetition_lvl -= 1;
            }
        }
    }

    pub(in crate::streaming_fast::file_sinks) fn get_max_repetition_lvl(&self) -> i16 {
        self.max_repetition_lvl
    }

    pub(in crate::streaming_fast::file_sinks) fn get_store(&self, repetition: &Repetition) -> Option<RepetitionAndDefinitionLvlStore> {
        match repetition {
            Repetition::REQUIRED => {
                if self.max_definition_lvl == 0 {
                    None
                } else {
                    Some(RepetitionAndDefinitionLvlStore {
                        definition_lvls: vec![],
                        repetition_lvls: vec![],
                        max_repetition_lvl: self.max_repetition_lvl
                    })
                }
            }
            Repetition::OPTIONAL => {
                Some(RepetitionAndDefinitionLvlStore {
                    definition_lvls: vec![],
                    repetition_lvls: vec![],
                    max_repetition_lvl: self.max_repetition_lvl
                })
            }
            Repetition::REPEATED => {
                Some(RepetitionAndDefinitionLvlStore {
                    definition_lvls: vec![],
                    repetition_lvls: vec![],
                    max_repetition_lvl: self.max_repetition_lvl + 1,
                })
            }
        }
    }
}