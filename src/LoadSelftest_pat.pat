PATTERN_NAME: LoadSelftest_pat
PATTERN_COMMENT: Load a selftest into the RAM using Auto Incremental method

PATTERN_SENDREF: DataSendRef, TestInterfaceInputPin

"Use <SELECT 0> for data regarding the selftest code"

MAIN: LoadSelftest_p
    SELECT: 0
    RESETFLAG: Fx
    
    PAUSE       "Pause to configure the execution (or not) of the TI_DutInit_sp subroutine"
    JUMP_COND: SkipDutInit_l, F1
    PAUSE       "Pause for <JSR TI_DutInit_sp> conditional configurations."
    SUB: TI_DutInit_sp
    JUMP: EndDutInit_l
    
    LABEL: SkipDutInit_l
    PAUSE       "Pause for <JSR TI_DutInit_sp> conditional configurations."
    SUB: TI_DutInitHotPluggingLightAndHaltCpu_sp
    
    LABEL: EndDutInit_l
    RESETFLAG: Fx
    PAUSE       "Pause for AutoInc algorithm configuration."
    
    "Program the selftest code using Auto Increment method"
    LABEL: AUTO_INC
    SUB: TI_EnableAutoIncrement_sp
    " F3 is set by c++ to skip code if the number of data to program is less than 1 auto increment buffer size."
    " F2 is set by c++ to skip code if the number of data to program is less than 2 auto increment buffer size."
    JUMP_COND: WriteRemainingData_l, F3
    CLOOP: LOOP_PAGES_UPDATE, 2   "Counter to update via C++ code with the number of time the auto increment buffer is used in the .hex code."
        TI_SendAddress_sp
        LOOP: LOOP_CURRENT_PAGE, GenericCoreArm::AhbAp::AUTO_INC_BUFFER_SIZE_W   "Counter is used for the auto increment buffer size (number of 32bits data)"
            TI_SendDataWord_sp
        END_LOOP
    END_CLOOP: F2     "Is F2 is set to false in the C++ code, end the loop. Used to make a loop of 1."
    "Process the remaining data (number of data below the auto increment buffer size)"
    LABEL: WriteRemainingData_l
    TI_SendAddress_sp
    LOOP: UPDATE_NWORDS, 2   "Counter to update via C++ code with the number remaining data."
        TI_SendDataWord_sp
    END_LOOP
    SUB: TI_DisableAutoIncrement_sp
    
END_MAIN_SINGLE
