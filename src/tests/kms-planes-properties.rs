use super::prelude::*;

#[cgt_test_with_fd(capabilities = [UniversalPlanes])]
fn test_kms_planes_properties_in_format(dev: BorrowedFd<'_>) -> Result<(), TestError> {
    for plane_id in find_planes(dev)? {
        let mut formats_found = false;

        for (id, _value) in find_properties_for_object(dev, plane_id)? {
            let mut count = drm_mode_get_property {
                prop_id: id,

                ..Default::default()
            };

            unsafe { drm_ioctl_mode_get_property(dev.as_raw_fd(), &mut count) }?;

            let prop_name = std::str::from_utf8(&count.name)
                .unwrap()
                .trim_end_matches(char::from(0));

            if prop_name == "IN_FORMATS" {
                formats_found = true;
            }
        }

        cgt_assert!(formats_found);
    }

    Ok(())
}
