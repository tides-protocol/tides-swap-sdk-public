// @generated
impl serde::Serialize for DeploymentInfoRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("tides.sui.hub.v1.DeploymentInfoRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeploymentInfoRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeploymentInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.DeploymentInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeploymentInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(DeploymentInfoRequest {
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.DeploymentInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeploymentInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.deployment_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.DeploymentInfoResponse", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if let Some(v) = self.deployment_info.as_ref() {
            struct_ser.serialize_field("deploymentInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeploymentInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "deployment_info",
            "deploymentInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            DeploymentInfo,
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
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "deploymentInfo" | "deployment_info" => Ok(GeneratedField::DeploymentInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeploymentInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.DeploymentInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeploymentInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut deployment_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DeploymentInfo => {
                            if deployment_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deploymentInfo"));
                            }
                            deployment_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DeploymentInfoResponse {
                    chain_id: chain_id__.unwrap_or_default(),
                    deployment_info: deployment_info__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.DeploymentInfoResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAssetsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("tides.sui.hub.v1.ListAssetsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAssetsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAssetsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.ListAssetsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAssetsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ListAssetsRequest {
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.ListAssetsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListAssetsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.assets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.ListAssetsResponse", len)?;
        if !self.assets.is_empty() {
            struct_ser.serialize_field("assets", &self.assets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListAssetsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "assets",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Assets,
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
                            "assets" => Ok(GeneratedField::Assets),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListAssetsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.ListAssetsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListAssetsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut assets__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Assets => {
                            if assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("assets"));
                            }
                            assets__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListAssetsResponse {
                    assets: assets__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.ListAssetsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NativeOracleType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::PushOracle => "PushOracle",
            Self::PullOracle => "PullOracle",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for NativeOracleType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PushOracle",
            "PullOracle",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NativeOracleType;

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
                    "PushOracle" => Ok(NativeOracleType::PushOracle),
                    "PullOracle" => Ok(NativeOracleType::PullOracle),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for NativeQuotePayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.featured_assets.is_empty() {
            len += 1;
        }
        if self.oracle_type != 0 {
            len += 1;
        }
        if !self.package_id.is_empty() {
            len += 1;
        }
        if self.oracle_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.NativeQuotePayload", len)?;
        if !self.featured_assets.is_empty() {
            struct_ser.serialize_field("featuredAssets", &self.featured_assets)?;
        }
        if self.oracle_type != 0 {
            let v = NativeOracleType::try_from(self.oracle_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.oracle_type)))?;
            struct_ser.serialize_field("oracleType", &v)?;
        }
        if !self.package_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("packageId", pbjson::private::base64::encode(&self.package_id).as_str())?;
        }
        if let Some(v) = self.oracle_id.as_ref() {
            struct_ser.serialize_field("oracleId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NativeQuotePayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "featured_assets",
            "featuredAssets",
            "oracle_type",
            "oracleType",
            "package_id",
            "packageId",
            "oracle_id",
            "oracleId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeaturedAssets,
            OracleType,
            PackageId,
            OracleId,
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
                            "featuredAssets" | "featured_assets" => Ok(GeneratedField::FeaturedAssets),
                            "oracleType" | "oracle_type" => Ok(GeneratedField::OracleType),
                            "packageId" | "package_id" => Ok(GeneratedField::PackageId),
                            "oracleId" | "oracle_id" => Ok(GeneratedField::OracleId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NativeQuotePayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.NativeQuotePayload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NativeQuotePayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut featured_assets__ = None;
                let mut oracle_type__ = None;
                let mut package_id__ = None;
                let mut oracle_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::FeaturedAssets => {
                            if featured_assets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("featuredAssets"));
                            }
                            featured_assets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OracleType => {
                            if oracle_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleType"));
                            }
                            oracle_type__ = Some(map_.next_value::<NativeOracleType>()? as i32);
                        }
                        GeneratedField::PackageId => {
                            if package_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packageId"));
                            }
                            package_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OracleId => {
                            if oracle_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oracleId"));
                            }
                            oracle_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(NativeQuotePayload {
                    featured_assets: featured_assets__.unwrap_or_default(),
                    oracle_type: oracle_type__.unwrap_or_default(),
                    package_id: package_id__.unwrap_or_default(),
                    oracle_id: oracle_id__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.NativeQuotePayload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PythConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pyth_state_id.is_some() {
            len += 1;
        }
        if !self.pyth_package_id.is_empty() {
            len += 1;
        }
        if self.wormhole_state_id.is_some() {
            len += 1;
        }
        if !self.wormhole_package_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.PythConfig", len)?;
        if let Some(v) = self.pyth_state_id.as_ref() {
            struct_ser.serialize_field("pythStateId", v)?;
        }
        if !self.pyth_package_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pythPackageId", pbjson::private::base64::encode(&self.pyth_package_id).as_str())?;
        }
        if let Some(v) = self.wormhole_state_id.as_ref() {
            struct_ser.serialize_field("wormholeStateId", v)?;
        }
        if !self.wormhole_package_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("wormholePackageId", pbjson::private::base64::encode(&self.wormhole_package_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PythConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pyth_state_id",
            "pythStateId",
            "pyth_package_id",
            "pythPackageId",
            "wormhole_state_id",
            "wormholeStateId",
            "wormhole_package_id",
            "wormholePackageId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PythStateId,
            PythPackageId,
            WormholeStateId,
            WormholePackageId,
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
                            "pythStateId" | "pyth_state_id" => Ok(GeneratedField::PythStateId),
                            "pythPackageId" | "pyth_package_id" => Ok(GeneratedField::PythPackageId),
                            "wormholeStateId" | "wormhole_state_id" => Ok(GeneratedField::WormholeStateId),
                            "wormholePackageId" | "wormhole_package_id" => Ok(GeneratedField::WormholePackageId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PythConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.PythConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PythConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pyth_state_id__ = None;
                let mut pyth_package_id__ = None;
                let mut wormhole_state_id__ = None;
                let mut wormhole_package_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PythStateId => {
                            if pyth_state_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pythStateId"));
                            }
                            pyth_state_id__ = map_.next_value()?;
                        }
                        GeneratedField::PythPackageId => {
                            if pyth_package_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pythPackageId"));
                            }
                            pyth_package_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::WormholeStateId => {
                            if wormhole_state_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wormholeStateId"));
                            }
                            wormhole_state_id__ = map_.next_value()?;
                        }
                        GeneratedField::WormholePackageId => {
                            if wormhole_package_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("wormholePackageId"));
                            }
                            wormhole_package_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PythConfig {
                    pyth_state_id: pyth_state_id__,
                    pyth_package_id: pyth_package_id__.unwrap_or_default(),
                    wormhole_state_id: wormhole_state_id__,
                    wormhole_package_id: wormhole_package_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.PythConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PythPriceUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.reserve_array_index != 0 {
            len += 1;
        }
        if self.price_info_object_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.PythPriceUpdate", len)?;
        if self.reserve_array_index != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("reserveArrayIndex", ToString::to_string(&self.reserve_array_index).as_str())?;
        }
        if let Some(v) = self.price_info_object_id.as_ref() {
            struct_ser.serialize_field("priceInfoObjectId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PythPriceUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reserve_array_index",
            "reserveArrayIndex",
            "price_info_object_id",
            "priceInfoObjectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReserveArrayIndex,
            PriceInfoObjectId,
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
                            "reserveArrayIndex" | "reserve_array_index" => Ok(GeneratedField::ReserveArrayIndex),
                            "priceInfoObjectId" | "price_info_object_id" => Ok(GeneratedField::PriceInfoObjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PythPriceUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.PythPriceUpdate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PythPriceUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reserve_array_index__ = None;
                let mut price_info_object_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReserveArrayIndex => {
                            if reserve_array_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reserveArrayIndex"));
                            }
                            reserve_array_index__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PriceInfoObjectId => {
                            if price_info_object_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceInfoObjectId"));
                            }
                            price_info_object_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PythPriceUpdate {
                    reserve_array_index: reserve_array_index__.unwrap_or_default(),
                    price_info_object_id: price_info_object_id__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.PythPriceUpdate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PythPriceUpdatePayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.price_updates.is_empty() {
            len += 1;
        }
        if self.update_price_fee != 0 {
            len += 1;
        }
        if !self.update.is_empty() {
            len += 1;
        }
        if !self.vaa.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.PythPriceUpdatePayload", len)?;
        if !self.price_updates.is_empty() {
            struct_ser.serialize_field("priceUpdates", &self.price_updates)?;
        }
        if self.update_price_fee != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("updatePriceFee", ToString::to_string(&self.update_price_fee).as_str())?;
        }
        if !self.update.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("update", pbjson::private::base64::encode(&self.update).as_str())?;
        }
        if !self.vaa.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("vaa", pbjson::private::base64::encode(&self.vaa).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PythPriceUpdatePayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_updates",
            "priceUpdates",
            "update_price_fee",
            "updatePriceFee",
            "update",
            "vaa",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceUpdates,
            UpdatePriceFee,
            Update,
            Vaa,
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
                            "priceUpdates" | "price_updates" => Ok(GeneratedField::PriceUpdates),
                            "updatePriceFee" | "update_price_fee" => Ok(GeneratedField::UpdatePriceFee),
                            "update" => Ok(GeneratedField::Update),
                            "vaa" => Ok(GeneratedField::Vaa),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PythPriceUpdatePayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.PythPriceUpdatePayload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PythPriceUpdatePayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_updates__ = None;
                let mut update_price_fee__ = None;
                let mut update__ = None;
                let mut vaa__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceUpdates => {
                            if price_updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceUpdates"));
                            }
                            price_updates__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdatePriceFee => {
                            if update_price_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatePriceFee"));
                            }
                            update_price_fee__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Update => {
                            if update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            update__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Vaa => {
                            if vaa__.is_some() {
                                return Err(serde::de::Error::duplicate_field("vaa"));
                            }
                            vaa__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PythPriceUpdatePayload {
                    price_updates: price_updates__.unwrap_or_default(),
                    update_price_fee: update_price_fee__.unwrap_or_default(),
                    update: update__.unwrap_or_default(),
                    vaa: vaa__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.PythPriceUpdatePayload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.input_amount != 0 {
            len += 1;
        }
        if self.output_amount != 0 {
            len += 1;
        }
        if self.output_floor != 0 {
            len += 1;
        }
        if self.rfq_account_id.is_some() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if self.protected_margin_account_id.is_some() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        if self.expiry_timestamp_unix_ms != 0 {
            len += 1;
        }
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.Quote", len)?;
        if self.input_amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("inputAmount", ToString::to_string(&self.input_amount).as_str())?;
        }
        if self.output_amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("outputAmount", ToString::to_string(&self.output_amount).as_str())?;
        }
        if self.output_floor != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("outputFloor", ToString::to_string(&self.output_floor).as_str())?;
        }
        if let Some(v) = self.rfq_account_id.as_ref() {
            struct_ser.serialize_field("rfqAccountId", v)?;
        }
        if !self.nonce.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", pbjson::private::base64::encode(&self.nonce).as_str())?;
        }
        if let Some(v) = self.protected_margin_account_id.as_ref() {
            struct_ser.serialize_field("protectedMarginAccountId", v)?;
        }
        if !self.signature.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        if self.expiry_timestamp_unix_ms != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("expiryTimestampUnixMs", ToString::to_string(&self.expiry_timestamp_unix_ms).as_str())?;
        }
        if let Some(v) = self.payload.as_ref() {
            match v {
                quote::Payload::NativePayload(v) => {
                    struct_ser.serialize_field("nativePayload", v)?;
                }
                quote::Payload::SuilendPayload(v) => {
                    struct_ser.serialize_field("suilendPayload", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "input_amount",
            "inputAmount",
            "output_amount",
            "outputAmount",
            "output_floor",
            "outputFloor",
            "rfq_account_id",
            "rfqAccountId",
            "nonce",
            "protected_margin_account_id",
            "protectedMarginAccountId",
            "signature",
            "expiry_timestamp_unix_ms",
            "expiryTimestampUnixMs",
            "native_payload",
            "nativePayload",
            "suilend_payload",
            "suilendPayload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InputAmount,
            OutputAmount,
            OutputFloor,
            RfqAccountId,
            Nonce,
            ProtectedMarginAccountId,
            Signature,
            ExpiryTimestampUnixMs,
            NativePayload,
            SuilendPayload,
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
                            "inputAmount" | "input_amount" => Ok(GeneratedField::InputAmount),
                            "outputAmount" | "output_amount" => Ok(GeneratedField::OutputAmount),
                            "outputFloor" | "output_floor" => Ok(GeneratedField::OutputFloor),
                            "rfqAccountId" | "rfq_account_id" => Ok(GeneratedField::RfqAccountId),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "protectedMarginAccountId" | "protected_margin_account_id" => Ok(GeneratedField::ProtectedMarginAccountId),
                            "signature" => Ok(GeneratedField::Signature),
                            "expiryTimestampUnixMs" | "expiry_timestamp_unix_ms" => Ok(GeneratedField::ExpiryTimestampUnixMs),
                            "nativePayload" | "native_payload" => Ok(GeneratedField::NativePayload),
                            "suilendPayload" | "suilend_payload" => Ok(GeneratedField::SuilendPayload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.Quote")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Quote, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut input_amount__ = None;
                let mut output_amount__ = None;
                let mut output_floor__ = None;
                let mut rfq_account_id__ = None;
                let mut nonce__ = None;
                let mut protected_margin_account_id__ = None;
                let mut signature__ = None;
                let mut expiry_timestamp_unix_ms__ = None;
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InputAmount => {
                            if input_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputAmount"));
                            }
                            input_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OutputAmount => {
                            if output_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputAmount"));
                            }
                            output_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OutputFloor => {
                            if output_floor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputFloor"));
                            }
                            output_floor__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RfqAccountId => {
                            if rfq_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rfqAccountId"));
                            }
                            rfq_account_id__ = map_.next_value()?;
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtectedMarginAccountId => {
                            if protected_margin_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protectedMarginAccountId"));
                            }
                            protected_margin_account_id__ = map_.next_value()?;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExpiryTimestampUnixMs => {
                            if expiry_timestamp_unix_ms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiryTimestampUnixMs"));
                            }
                            expiry_timestamp_unix_ms__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NativePayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nativePayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(quote::Payload::NativePayload)
;
                        }
                        GeneratedField::SuilendPayload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suilendPayload"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(quote::Payload::SuilendPayload)
;
                        }
                    }
                }
                Ok(Quote {
                    input_amount: input_amount__.unwrap_or_default(),
                    output_amount: output_amount__.unwrap_or_default(),
                    output_floor: output_floor__.unwrap_or_default(),
                    rfq_account_id: rfq_account_id__,
                    nonce: nonce__.unwrap_or_default(),
                    protected_margin_account_id: protected_margin_account_id__,
                    signature: signature__.unwrap_or_default(),
                    expiry_timestamp_unix_ms: expiry_timestamp_unix_ms__.unwrap_or_default(),
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.Quote", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuoteTradeExactInRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.input_type.is_empty() {
            len += 1;
        }
        if self.input_amount != 0 {
            len += 1;
        }
        if !self.output_type.is_empty() {
            len += 1;
        }
        if self.min_output_amount.is_some() {
            len += 1;
        }
        if self.rfq_timeout.is_some() {
            len += 1;
        }
        if self.quote_expiry.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.QuoteTradeExactInRequest", len)?;
        if !self.input_type.is_empty() {
            struct_ser.serialize_field("inputType", &self.input_type)?;
        }
        if self.input_amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("inputAmount", ToString::to_string(&self.input_amount).as_str())?;
        }
        if !self.output_type.is_empty() {
            struct_ser.serialize_field("outputType", &self.output_type)?;
        }
        if let Some(v) = self.min_output_amount.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("minOutputAmount", ToString::to_string(&v).as_str())?;
        }
        if let Some(v) = self.rfq_timeout.as_ref() {
            struct_ser.serialize_field("rfqTimeout", v)?;
        }
        if let Some(v) = self.quote_expiry.as_ref() {
            struct_ser.serialize_field("quoteExpiry", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteTradeExactInRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "input_type",
            "inputType",
            "input_amount",
            "inputAmount",
            "output_type",
            "outputType",
            "min_output_amount",
            "minOutputAmount",
            "rfq_timeout",
            "rfqTimeout",
            "quote_expiry",
            "quoteExpiry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InputType,
            InputAmount,
            OutputType,
            MinOutputAmount,
            RfqTimeout,
            QuoteExpiry,
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
                            "inputType" | "input_type" => Ok(GeneratedField::InputType),
                            "inputAmount" | "input_amount" => Ok(GeneratedField::InputAmount),
                            "outputType" | "output_type" => Ok(GeneratedField::OutputType),
                            "minOutputAmount" | "min_output_amount" => Ok(GeneratedField::MinOutputAmount),
                            "rfqTimeout" | "rfq_timeout" => Ok(GeneratedField::RfqTimeout),
                            "quoteExpiry" | "quote_expiry" => Ok(GeneratedField::QuoteExpiry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuoteTradeExactInRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.QuoteTradeExactInRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuoteTradeExactInRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut input_type__ = None;
                let mut input_amount__ = None;
                let mut output_type__ = None;
                let mut min_output_amount__ = None;
                let mut rfq_timeout__ = None;
                let mut quote_expiry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InputType => {
                            if input_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputType"));
                            }
                            input_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InputAmount => {
                            if input_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputAmount"));
                            }
                            input_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OutputType => {
                            if output_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputType"));
                            }
                            output_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MinOutputAmount => {
                            if min_output_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minOutputAmount"));
                            }
                            min_output_amount__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::RfqTimeout => {
                            if rfq_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rfqTimeout"));
                            }
                            rfq_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::QuoteExpiry => {
                            if quote_expiry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteExpiry"));
                            }
                            quote_expiry__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuoteTradeExactInRequest {
                    input_type: input_type__.unwrap_or_default(),
                    input_amount: input_amount__.unwrap_or_default(),
                    output_type: output_type__.unwrap_or_default(),
                    min_output_amount: min_output_amount__,
                    rfq_timeout: rfq_timeout__,
                    quote_expiry: quote_expiry__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.QuoteTradeExactInRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuoteTradeExactInResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rfq.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.QuoteTradeExactInResponse", len)?;
        if let Some(v) = self.rfq.as_ref() {
            struct_ser.serialize_field("rfq", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteTradeExactInResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rfq",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rfq,
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
                            "rfq" => Ok(GeneratedField::Rfq),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuoteTradeExactInResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.QuoteTradeExactInResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuoteTradeExactInResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rfq__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rfq => {
                            if rfq__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rfq"));
                            }
                            rfq__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuoteTradeExactInResponse {
                    rfq: rfq__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.QuoteTradeExactInResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuoteTradeExactOutRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.input_type.is_empty() {
            len += 1;
        }
        if self.max_input_amount.is_some() {
            len += 1;
        }
        if !self.output_type.is_empty() {
            len += 1;
        }
        if self.output_amount != 0 {
            len += 1;
        }
        if self.rfq_timeout.is_some() {
            len += 1;
        }
        if self.quote_expiry.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.QuoteTradeExactOutRequest", len)?;
        if !self.input_type.is_empty() {
            struct_ser.serialize_field("inputType", &self.input_type)?;
        }
        if let Some(v) = self.max_input_amount.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxInputAmount", ToString::to_string(&v).as_str())?;
        }
        if !self.output_type.is_empty() {
            struct_ser.serialize_field("outputType", &self.output_type)?;
        }
        if self.output_amount != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("outputAmount", ToString::to_string(&self.output_amount).as_str())?;
        }
        if let Some(v) = self.rfq_timeout.as_ref() {
            struct_ser.serialize_field("rfqTimeout", v)?;
        }
        if let Some(v) = self.quote_expiry.as_ref() {
            struct_ser.serialize_field("quoteExpiry", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteTradeExactOutRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "input_type",
            "inputType",
            "max_input_amount",
            "maxInputAmount",
            "output_type",
            "outputType",
            "output_amount",
            "outputAmount",
            "rfq_timeout",
            "rfqTimeout",
            "quote_expiry",
            "quoteExpiry",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            InputType,
            MaxInputAmount,
            OutputType,
            OutputAmount,
            RfqTimeout,
            QuoteExpiry,
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
                            "inputType" | "input_type" => Ok(GeneratedField::InputType),
                            "maxInputAmount" | "max_input_amount" => Ok(GeneratedField::MaxInputAmount),
                            "outputType" | "output_type" => Ok(GeneratedField::OutputType),
                            "outputAmount" | "output_amount" => Ok(GeneratedField::OutputAmount),
                            "rfqTimeout" | "rfq_timeout" => Ok(GeneratedField::RfqTimeout),
                            "quoteExpiry" | "quote_expiry" => Ok(GeneratedField::QuoteExpiry),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuoteTradeExactOutRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.QuoteTradeExactOutRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuoteTradeExactOutRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut input_type__ = None;
                let mut max_input_amount__ = None;
                let mut output_type__ = None;
                let mut output_amount__ = None;
                let mut rfq_timeout__ = None;
                let mut quote_expiry__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::InputType => {
                            if input_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inputType"));
                            }
                            input_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxInputAmount => {
                            if max_input_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxInputAmount"));
                            }
                            max_input_amount__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::OutputType => {
                            if output_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputType"));
                            }
                            output_type__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OutputAmount => {
                            if output_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outputAmount"));
                            }
                            output_amount__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RfqTimeout => {
                            if rfq_timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rfqTimeout"));
                            }
                            rfq_timeout__ = map_.next_value()?;
                        }
                        GeneratedField::QuoteExpiry => {
                            if quote_expiry__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quoteExpiry"));
                            }
                            quote_expiry__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuoteTradeExactOutRequest {
                    input_type: input_type__.unwrap_or_default(),
                    max_input_amount: max_input_amount__,
                    output_type: output_type__.unwrap_or_default(),
                    output_amount: output_amount__.unwrap_or_default(),
                    rfq_timeout: rfq_timeout__,
                    quote_expiry: quote_expiry__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.QuoteTradeExactOutRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuoteTradeExactOutResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.rfq.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.QuoteTradeExactOutResponse", len)?;
        if let Some(v) = self.rfq.as_ref() {
            struct_ser.serialize_field("rfq", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuoteTradeExactOutResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rfq",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rfq,
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
                            "rfq" => Ok(GeneratedField::Rfq),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuoteTradeExactOutResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.QuoteTradeExactOutResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuoteTradeExactOutResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rfq__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rfq => {
                            if rfq__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rfq"));
                            }
                            rfq__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuoteTradeExactOutResponse {
                    rfq: rfq__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.QuoteTradeExactOutResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Rfq {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trade_id.is_empty() {
            len += 1;
        }
        if self.quote.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.Rfq", len)?;
        if !self.trade_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tradeId", pbjson::private::base64::encode(&self.trade_id).as_str())?;
        }
        if let Some(v) = self.quote.as_ref() {
            struct_ser.serialize_field("quote", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Rfq {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trade_id",
            "tradeId",
            "quote",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TradeId,
            Quote,
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
                            "tradeId" | "trade_id" => Ok(GeneratedField::TradeId),
                            "quote" => Ok(GeneratedField::Quote),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Rfq;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.Rfq")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Rfq, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trade_id__ = None;
                let mut quote__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TradeId => {
                            if trade_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradeId"));
                            }
                            trade_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Quote => {
                            if quote__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quote"));
                            }
                            quote__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Rfq {
                    trade_id: trade_id__.unwrap_or_default(),
                    quote: quote__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.Rfq", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamRfqWinnersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("tides.sui.hub.v1.StreamRfqWinnersRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamRfqWinnersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamRfqWinnersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.StreamRfqWinnersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamRfqWinnersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(StreamRfqWinnersRequest {
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.StreamRfqWinnersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamRfqWinnersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.trade_id.is_empty() {
            len += 1;
        }
        if self.rfq_winner_account_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.StreamRfqWinnersResponse", len)?;
        if !self.trade_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("tradeId", pbjson::private::base64::encode(&self.trade_id).as_str())?;
        }
        if let Some(v) = self.rfq_winner_account_id.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("rfqWinnerAccountId", pbjson::private::base64::encode(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StreamRfqWinnersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "trade_id",
            "tradeId",
            "rfq_winner_account_id",
            "rfqWinnerAccountId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TradeId,
            RfqWinnerAccountId,
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
                            "tradeId" | "trade_id" => Ok(GeneratedField::TradeId),
                            "rfqWinnerAccountId" | "rfq_winner_account_id" => Ok(GeneratedField::RfqWinnerAccountId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamRfqWinnersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.StreamRfqWinnersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StreamRfqWinnersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut trade_id__ = None;
                let mut rfq_winner_account_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TradeId => {
                            if trade_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradeId"));
                            }
                            trade_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RfqWinnerAccountId => {
                            if rfq_winner_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rfqWinnerAccountId"));
                            }
                            rfq_winner_account_id__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::BytesDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(StreamRfqWinnersResponse {
                    trade_id: trade_id__.unwrap_or_default(),
                    rfq_winner_account_id: rfq_winner_account_id__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.StreamRfqWinnersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SuilendQuotePayload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.price_updates_payload.is_some() {
            len += 1;
        }
        if self.pyth_config.is_some() {
            len += 1;
        }
        if !self.suilend_package_id.is_empty() {
            len += 1;
        }
        if self.suilend_lending_market_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("tides.sui.hub.v1.SuilendQuotePayload", len)?;
        if let Some(v) = self.price_updates_payload.as_ref() {
            struct_ser.serialize_field("priceUpdatesPayload", v)?;
        }
        if let Some(v) = self.pyth_config.as_ref() {
            struct_ser.serialize_field("pythConfig", v)?;
        }
        if !self.suilend_package_id.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("suilendPackageId", pbjson::private::base64::encode(&self.suilend_package_id).as_str())?;
        }
        if let Some(v) = self.suilend_lending_market_id.as_ref() {
            struct_ser.serialize_field("suilendLendingMarketId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SuilendQuotePayload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "price_updates_payload",
            "priceUpdatesPayload",
            "pyth_config",
            "pythConfig",
            "suilend_package_id",
            "suilendPackageId",
            "suilend_lending_market_id",
            "suilendLendingMarketId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PriceUpdatesPayload,
            PythConfig,
            SuilendPackageId,
            SuilendLendingMarketId,
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
                            "priceUpdatesPayload" | "price_updates_payload" => Ok(GeneratedField::PriceUpdatesPayload),
                            "pythConfig" | "pyth_config" => Ok(GeneratedField::PythConfig),
                            "suilendPackageId" | "suilend_package_id" => Ok(GeneratedField::SuilendPackageId),
                            "suilendLendingMarketId" | "suilend_lending_market_id" => Ok(GeneratedField::SuilendLendingMarketId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SuilendQuotePayload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct tides.sui.hub.v1.SuilendQuotePayload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SuilendQuotePayload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut price_updates_payload__ = None;
                let mut pyth_config__ = None;
                let mut suilend_package_id__ = None;
                let mut suilend_lending_market_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PriceUpdatesPayload => {
                            if price_updates_payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("priceUpdatesPayload"));
                            }
                            price_updates_payload__ = map_.next_value()?;
                        }
                        GeneratedField::PythConfig => {
                            if pyth_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pythConfig"));
                            }
                            pyth_config__ = map_.next_value()?;
                        }
                        GeneratedField::SuilendPackageId => {
                            if suilend_package_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suilendPackageId"));
                            }
                            suilend_package_id__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SuilendLendingMarketId => {
                            if suilend_lending_market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suilendLendingMarketId"));
                            }
                            suilend_lending_market_id__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SuilendQuotePayload {
                    price_updates_payload: price_updates_payload__,
                    pyth_config: pyth_config__,
                    suilend_package_id: suilend_package_id__.unwrap_or_default(),
                    suilend_lending_market_id: suilend_lending_market_id__,
                })
            }
        }
        deserializer.deserialize_struct("tides.sui.hub.v1.SuilendQuotePayload", FIELDS, GeneratedVisitor)
    }
}
