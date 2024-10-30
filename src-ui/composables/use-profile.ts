import type { Profile } from '~/models/profile'
import { useAsyncData } from '#app'

// @ts-ignore
export default async function (): Promise<Ref<Profile>> {
  const route = useRoute()
  const { $profileService } = useNuxtApp()
  const toast = useToast()

  try {
    const { data: profile } = await useAsyncData(`profile-${route.params.profileId}`, () =>
      $profileService.getById(+route.params.profileId)
    )

    if (!profile.value) {
      // noinspection ExceptionCaughtLocallyJS
      throw 'Not found'
    }

    // @ts-ignore
    return profile
  } catch {
    toast.add({ id: 'profile-doesnt-exists', title: "Ce profil n'existe plus" })
    await navigateTo('/')
  }
}
