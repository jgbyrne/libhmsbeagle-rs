use bitflags::bitflags;

#[derive(Debug)]
pub struct InstanceDetails {
    pub resourceNumber: i32,
    pub resourceName: String,
    pub implName: String,
    pub implDescription: String,
    pub flags: Flags,
}

#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub description: String,
    pub supportFlags: Flags,
    pub requiredFlags: Flags,
}

#[derive(Debug)]
pub struct Operation {
    pub destinationPartials: i32,
    pub destinationScaleWrite: i32,
    pub destinationScaleRead: i32,
    pub child1Partials: i32,
    pub child1TransitionMatrix: i32,
    pub child2Partials: i32,
    pub child2TransitionMatrix: i32,
}

#[derive(Debug)]
pub struct OperationByPartition {
    pub destinationPartials: i32,
    pub destinationScaleWrite: i32,
    pub destinationScaleRead: i32,
    pub child1Partials: i32,
    pub child1TransitionMatrix: i32,
    pub child2Partials: i32,
    pub child2TransitionMatrix: i32,
    pub partition: i32,
    pub cumulativeScaleIndex: i32,
}

#[derive(PartialEq, Debug)]
pub enum ReturnCode {
    SUCCESS = 0,
    ERROR_GENERAL = -1,
    ERROR_OUT_OF_MEMORY = -2,
    ERROR_UNIDENTIFIED_EXCEPTION = -3,
    ERROR_UNINITIALIZED_INSTANCE = -4,
    ERROR_OUT_OF_RANGE = -5,
    ERROR_NO_RESOURCE = -6,
    ERROR_NO_IMPLEMENTATION = -7,
    ERROR_FLOATING_POINT = -8,
}

bitflags! {
    pub struct Flags: i64 {
        const PRECISION_SINGLE = 1 << 0;
        const PRECISION_DOUBLE = 1 << 1;
        const COMPUTATION_SYNCH = 1 << 2;
        const COMPUTATION_ASYNCH = 1 << 3;
        const EIGEN_REAL = 1 << 4;
        const EIGEN_COMPLEX = 1 << 5;
        const SCALNG_MANUAL = 1 << 6;
        const SCALING_AUTO = 1 << 7;
        const SCALING_ALWAYS = 1 << 8;
        const SCALING_DYNAMIC = 1 << 25;
        const SCALERS_RAW = 1 << 9;
        const SCALERS_LOG = 1 << 10;
        const INVEVEC_STANDARD = 1 << 20;
        const INVEVEC_TRANSPOSED = 1 << 21;
        const VECTOR_SSE = 1 << 11;
        const VECTOR_AVX = 1 << 24;
        const VECTOR_NONE = 1 << 12;
        const THREADING_CPP = 1 << 30;
        const THREADING_OPENMP = 1 << 13;
        const THREADING_NONE = 1 << 14;
        const PROCESSOR_CPU = 1 << 15;
        const PROCESSOR_GPU = 1 << 16;
        const PROCESSOR_FPGA = 1 << 17;
        const PROCESSOR_CELL = 1 << 18;
        const PROCESSOR_PHI = 1 << 19;
        const PROCESSOR_OTHER = 1 << 26;
        const FRAMEWORK_CUDA = 1 << 22;
        const FRAMEWORK_OPENCL = 1 << 23;
        const FRAMEWORK_CPU = 1 << 27;
        const PARALLELOPS_STREAMS = 1 << 28;
        const PARALLELOPS_GRID = 1 << 29;
    }
}

#[derive(Debug)]
pub enum OpCodes {
    OP_COUNT = 7,
    PARTITION_OP_COUNT = 9,
    OP_NONE = -1,
}
