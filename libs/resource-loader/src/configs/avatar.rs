use std::path::Path;

use serde::{Deserialize, Serialize};

use super::{free_path_ref, static_leak};

default_string! {
    BaseDir=>"avatars"
    Group=>"groups"
    User=>"users"
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, derivative::Derivative)]
#[derivative(Default)]
pub struct AvatarConfig {
    #[derivative(Default(value = r#"BaseDir::get_default()"#))]
    #[serde(default = "BaseDir::get_default")]
    #[serde(alias = "base")]
    base_dir: String,
    #[derivative(Default(value = r#"Group::get_default()"#))]
    #[serde(default = "Group::get_default")]
    group: String,
    #[derivative(Default(value = r#"User::get_default()"#))]
    #[serde(default = "User::get_default")]
    user: String,
}

/// # Panic
/// using string literal construct this struct will cause  
/// ***STATUS_HEAP_CORRUPTION***,
///
/// the internal `& 'static Path` comes from `Box::leak`
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct InnerAvatarConfig {
    pub group: &'static Path,
    pub user: &'static Path,
}
impl AvatarConfig {
    pub(crate) fn into_inner(self, root: &Path) -> InnerAvatarConfig {
        let avatar = root.join(&self.base_dir);

        let group = avatar.join(&self.group);
        let static_group = static_leak(group.into_boxed_path());

        let user = avatar.join(&self.user);
        let static_user = static_leak(user.into_boxed_path());

        InnerAvatarConfig {
            group: static_group,
            user: static_user,
        }
    }
}

impl Drop for InnerAvatarConfig {
    fn drop(&mut self) {
        free_path_ref(self.group);
        free_path_ref(self.user);
    }
}

#[cfg(test)]
mod test {

    use std::path::Path;

    use serde_json::json;

    use super::{AvatarConfig, BaseDir, Group, User};

    #[test]
    fn test_default_data() {
        let avatar = AvatarConfig::default();

        assert_eq!(
            avatar,
            AvatarConfig {
                base_dir: BaseDir::get_default(),
                group: Group::get_default(),
                user: User::get_default()
            }
        )
    }
    #[test]
    fn test_not_full() {
        let data = json! {
            {
                "base_dir":"avatar_cache",
                "group":"group_avatar"
            }
        };

        let avatar = serde_json::from_value::<AvatarConfig>(data).unwrap();

        assert_eq!(
            avatar,
            AvatarConfig {
                base_dir: "avatar_cache".into(),
                group: "group_avatar".into(),
                user: User::get_default()
            }
        )
    }

    #[test]
    fn test_inner_drop() {
        let avatar = AvatarConfig::default();

        let inner = avatar.into_inner(Path::new("gtk-qq"));

        assert_eq!(inner.group, Path::new("gtk-qq\\avatars\\groups"));
        assert_eq!(inner.user, Path::new("gtk-qq\\avatars\\users"))
    }
}
