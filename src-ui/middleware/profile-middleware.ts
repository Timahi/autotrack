export default defineNuxtRouteMiddleware(async (to) => {
  const profileId = Number(to.params.profileId)

  const { $profileService } = useNuxtApp()

  const { setProfile } = selectedProfile()

  try {
    const profile = await $profileService.getById(profileId)

    if (!profile) {
      return navigateTo('/')
    }

    setProfile(profile)
  } catch {
    return navigateTo('/')
  }
})
