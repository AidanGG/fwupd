// Copyright (C) 2023 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1+

#[derive(ToString)]
enum LogitechHidppFeature {
    Root                  = 0x0000,
    IFeatureSet           = 0x0001,
    IFirmwareInfo         = 0x0003,
    GetDeviceNameType     = 0x0005,
    DfuControl            = 0x00C1,
    DfuControlSigned      = 0x00C2,
    DfuControlBolt        = 0x00C3,
    Dfu                   = 0x00D0,
    BatteryLevelStatus    = 0x1000,
    UnifiedBattery        = 0x1004,
    KbdReprogrammableKeys = 0x1B00,
    SpecialKeysButtons    = 0x1B04,
    MousePointerBasic     = 0x2200,
    AdjustableDpi         = 0x2201,
    AdjustableReportRate  = 0x8060,
    ColorLedEffects       = 0x8070,
    OnboardProfiles       = 0x8100,
    MouseButtonSpy        = 0x8110,
}
