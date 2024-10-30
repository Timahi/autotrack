export class Vehicle {
  private constructor(
    public id: number,
    public brand: string,
    public model: string,
    public odometer: number,
    public odometerUpdatedAt: Date,
    public registration: string,
    public registrationYear: string,
    public serialNumber: string | null,
    public description: string | null,
    public profileId: number,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static from({
    id,
    brand,
    model,
    odometer,
    odometer_updated_at,
    registration,
    registration_year,
    serial_number,
    description,
    profile_id,
    created_at,
    updated_at,
  }: {
    id: number
    brand: string
    model: string
    odometer: number
    odometer_updated_at: string
    registration: string
    registration_year: string
    serial_number: string | null
    description: string | null
    profile_id: number
    created_at: string
    updated_at: string
  }) {
    return new Vehicle(
      id,
      brand,
      model,
      odometer,
      new Date(odometer_updated_at + 'Z'),
      registration,
      registration_year,
      serial_number,
      description,
      profile_id,
      new Date(created_at + 'Z'),
      new Date(updated_at + 'Z')
    )
  }
}

export class NewVehicle {
  private constructor(
    public brand: string,
    public model: string,
    public odometer: number,
    public odometerUpdatedAt: Date,
    public registration: string,
    public registrationYear: number,
    public profileId: number,
    public createdAt: Date,
    public updatedAt: Date,
    public serialNumber?: string,
    public description?: string
  ) {}

  static from({
    brand,
    model,
    odometer,
    odometerUpdatedAt,
    registration,
    registrationYear,
    serialNumber,
    description,
    profileId,
    createdAt,
    updatedAt,
  }: {
    brand: string
    model: string
    odometer: number
    odometerUpdatedAt: Date
    registration: string
    registrationYear: number
    serialNumber?: string
    description?: string
    profileId: number
    createdAt: Date
    updatedAt: Date
  }) {
    return new NewVehicle(
      brand,
      model,
      odometer,
      odometerUpdatedAt,
      registration,
      registrationYear,
      profileId,
      createdAt,
      updatedAt,
      serialNumber,
      description
    )
  }

  toJSON() {
    return {
      brand: this.brand,
      model: this.model,
      odometer: this.odometer,
      odometer_updated_at: this.odometerUpdatedAt.toISOString().slice(0, -1),
      registration: this.registration,
      registration_year: this.registrationYear,
      serial_number: this.serialNumber,
      description: this.description,
      profile_id: this.profileId,
      created_at: this.createdAt.toISOString().slice(0, -1),
      updated_at: this.updatedAt.toISOString().slice(0, -1),
    }
  }
}

export class EditVehicle {
  private constructor(
    public updatedAt: Date,
    public brand?: string,
    public model?: string,
    public odometer?: number,
    public registration?: string,
    public registrationYear?: number,
    public profileId?: number,
    public odometerUpdatedAt?: Date,
    public serialNumber?: string,
    public description?: string
  ) {}

  static from({
    brand,
    model,
    odometer,
    odometerUpdatedAt,
    registration,
    registrationYear,
    serialNumber,
    description,
    profileId,
    updatedAt,
  }: {
    brand?: string
    model?: string
    odometer?: number
    odometerUpdatedAt?: Date
    registration?: string
    registrationYear?: number
    serialNumber?: string
    description?: string
    profileId?: number
    updatedAt: Date
  }) {
    return new EditVehicle(
      updatedAt,
      brand,
      model,
      odometer,
      registration,
      registrationYear,
      profileId,
      odometerUpdatedAt,
      serialNumber,
      description
    )
  }

  toJSON() {
    return {
      brand: this.brand,
      model: this.model,
      odometer: this.odometer,
      odometer_updated_at: this.odometerUpdatedAt?.toISOString().slice(0, -1),
      registration: this.registration,
      registration_year: this.registrationYear,
      serial_number: this.serialNumber,
      description: this.description,
      profile_id: this.profileId,
      updated_at: this.updatedAt.toISOString().slice(0, -1),
    }
  }
}
