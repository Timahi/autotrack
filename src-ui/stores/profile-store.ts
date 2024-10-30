import type { Profile } from '~/models/profile'

export default defineStore('profile-store', () => {
  const profile = ref<Profile>(undefined as unknown as Profile)

  const setProfile = (newProfile: Profile) => {
    profile.value = newProfile
  }

  return { profile, setProfile }
})
