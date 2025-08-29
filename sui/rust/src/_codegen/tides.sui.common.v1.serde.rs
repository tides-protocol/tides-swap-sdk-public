// @generated
impl serde::Serialize for AssetInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.type_tag.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.common.v1.AssetInfo", len)?;
        if !self.type_tag.is_empty() {
            struct_ser.serialize_field("typeTag", &self.type_tag)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AssetInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type_tag",
            "typeTag",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TypeTag,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "typeTag" | "type_tag" => Ok(GeneratedField::TypeTag),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AssetInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.common.v1.AssetInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AssetInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut type_tag__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TypeTag => {
                            if type_tag__.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeTag"));
                            }
                            type_tag__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(AssetInfo {
                    type_tag: type_tag__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.common.v1.AssetInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeploymentInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.package.is_empty() {
            len += 1;
        }
        if !self.global_info.is_empty() {
            len += 1;
        }
        if !self.admin_cap.is_empty() {
            len += 1;
        }
        if !self.risk_params.is_empty() {
            len += 1;
        }
        if !self.trusted_push_oracle.is_empty() {
            len += 1;
        }
        if !self.trusted_push_oracle_cap.is_empty() {
            len += 1;
        }
        if !self.lending_pool_registry.is_empty() {
            len += 1;
        }
        if !self.future_tokens_registry.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.common.v1.DeploymentInfo", len)?;
        if !self.package.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("package", pbjson::private::base64::encode(&self.package).as_str())?;
        }
        if !self.global_info.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("globalInfo", pbjson::private::base64::encode(&self.global_info).as_str())?;
        }
        if !self.admin_cap.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("adminCap", pbjson::private::base64::encode(&self.admin_cap).as_str())?;
        }
        if !self.risk_params.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("riskParams", pbjson::private::base64::encode(&self.risk_params).as_str())?;
        }
        if !self.trusted_push_oracle.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("trustedPushOracle", pbjson::private::base64::encode(&self.trusted_push_oracle).as_str())?;
        }
        if !self.trusted_push_oracle_cap.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("trustedPushOracleCap", pbjson::private::base64::encode(&self.trusted_push_oracle_cap).as_str())?;
        }
        if !self.lending_pool_registry.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("lendingPoolRegistry", pbjson::private::base64::encode(&self.lending_pool_registry).as_str())?;
        }
        if !self.future_tokens_registry.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("futureTokensRegistry", pbjson::private::base64::encode(&self.future_tokens_registry).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeploymentInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "package",
            "global_info",
            "globalInfo",
            "admin_cap",
            "adminCap",
            "risk_params",
            "riskParams",
            "trusted_push_oracle",
            "trustedPushOracle",
            "trusted_push_oracle_cap",
            "trustedPushOracleCap",
            "lending_pool_registry",
            "lendingPoolRegistry",
            "future_tokens_registry",
            "futureTokensRegistry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Package,
            GlobalInfo,
            AdminCap,
            RiskParams,
            TrustedPushOracle,
            TrustedPushOracleCap,
            LendingPoolRegistry,
            FutureTokensRegistry,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "package" => Ok(GeneratedField::Package),
                            "globalInfo" | "global_info" => Ok(GeneratedField::GlobalInfo),
                            "adminCap" | "admin_cap" => Ok(GeneratedField::AdminCap),
                            "riskParams" | "risk_params" => Ok(GeneratedField::RiskParams),
                            "trustedPushOracle" | "trusted_push_oracle" => Ok(GeneratedField::TrustedPushOracle),
                            "trustedPushOracleCap" | "trusted_push_oracle_cap" => Ok(GeneratedField::TrustedPushOracleCap),
                            "lendingPoolRegistry" | "lending_pool_registry" => Ok(GeneratedField::LendingPoolRegistry),
                            "futureTokensRegistry" | "future_tokens_registry" => Ok(GeneratedField::FutureTokensRegistry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeploymentInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.common.v1.DeploymentInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeploymentInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut package__ = None;
                let mut global_info__ = None;
                let mut admin_cap__ = None;
                let mut risk_params__ = None;
                let mut trusted_push_oracle__ = None;
                let mut trusted_push_oracle_cap__ = None;
                let mut lending_pool_registry__ = None;
                let mut future_tokens_registry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Package => {
                            if package__.is_some() {
                                return Err(serde::de::Error::duplicate_field("package"));
                            }
                            package__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GlobalInfo => {
                            if global_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("globalInfo"));
                            }
                            global_info__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AdminCap => {
                            if admin_cap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminCap"));
                            }
                            admin_cap__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RiskParams => {
                            if risk_params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("riskParams"));
                            }
                            risk_params__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrustedPushOracle => {
                            if trusted_push_oracle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedPushOracle"));
                            }
                            trusted_push_oracle__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TrustedPushOracleCap => {
                            if trusted_push_oracle_cap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trustedPushOracleCap"));
                            }
                            trusted_push_oracle_cap__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LendingPoolRegistry => {
                            if lending_pool_registry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lendingPoolRegistry"));
                            }
                            lending_pool_registry__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::FutureTokensRegistry => {
                            if future_tokens_registry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("futureTokensRegistry"));
                            }
                            future_tokens_registry__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DeploymentInfo {
                    package: package__.unwrap_or_default(),
                    global_info: global_info__.unwrap_or_default(),
                    admin_cap: admin_cap__.unwrap_or_default(),
                    risk_params: risk_params__.unwrap_or_default(),
                    trusted_push_oracle: trusted_push_oracle__.unwrap_or_default(),
                    trusted_push_oracle_cap: trusted_push_oracle_cap__.unwrap_or_default(),
                    lending_pool_registry: lending_pool_registry__.unwrap_or_default(),
                    future_tokens_registry: future_tokens_registry__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.common.v1.DeploymentInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MarginAccountType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "MARGIN_ACCOUNT_TYPE_UNSPECIFIED",
            Self::Native => "MARGIN_ACCOUNT_TYPE_NATIVE",
            Self::Suilend => "MARGIN_ACCOUNT_TYPE_SUILEND",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for MarginAccountType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "MARGIN_ACCOUNT_TYPE_UNSPECIFIED",
            "MARGIN_ACCOUNT_TYPE_NATIVE",
            "MARGIN_ACCOUNT_TYPE_SUILEND",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MarginAccountType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "MARGIN_ACCOUNT_TYPE_UNSPECIFIED" => Ok(MarginAccountType::Unspecified),
                    "MARGIN_ACCOUNT_TYPE_NATIVE" => Ok(MarginAccountType::Native),
                    "MARGIN_ACCOUNT_TYPE_SUILEND" => Ok(MarginAccountType::Suilend),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for SharedObject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.initial_shared_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.common.v1.SharedObject", len)?;
        if !self.id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("id", pbjson::private::base64::encode(&self.id).as_str())?;
        }
        if self.initial_shared_version != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("initialSharedVersion", ToString::to_string(&self.initial_shared_version).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SharedObject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "initial_shared_version",
            "initialSharedVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            InitialSharedVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            "initialSharedVersion" | "initial_shared_version" => Ok(GeneratedField::InitialSharedVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SharedObject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.common.v1.SharedObject")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SharedObject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut initial_shared_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InitialSharedVersion => {
                            if initial_shared_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialSharedVersion"));
                            }
                            initial_shared_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(SharedObject {
                    id: id__.unwrap_or_default(),
                    initial_shared_version: initial_shared_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.common.v1.SharedObject", FIELDS, GeneratedVisitor)
    }
}
