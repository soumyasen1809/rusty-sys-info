use crate::sys_stats::{
    cpu::CpuMeasurements, disk::DiskStatMeasurements, memory::MemoryMeasurments,
    nvidia_gpu::NvidiaGpuMeasurements, socket::SocketStatMeasurements,
};

pub struct UIMeasurements {
    pub ui_cpu_data: CpuMeasurements,
    pub ui_memory_data: MemoryMeasurments,
    pub ui_disk_data: DiskStatMeasurements,
    pub ui_socket_data: SocketStatMeasurements,
    pub ui_nvidia_gpu_data: NvidiaGpuMeasurements,
}

impl UIMeasurements {
    pub fn ui_cpu_data(&self) -> &CpuMeasurements {
        &self.ui_cpu_data
    }

    pub fn ui_memory_data(&self) -> &MemoryMeasurments {
        &self.ui_memory_data
    }

    pub fn ui_disk_data(&self) -> &DiskStatMeasurements {
        &self.ui_disk_data
    }

    pub fn ui_socket_data(&self) -> &SocketStatMeasurements {
        &self.ui_socket_data
    }

    pub fn ui_nvidia_gpu_data(&self) -> &NvidiaGpuMeasurements {
        &self.ui_nvidia_gpu_data
    }
}

impl Default for UIMeasurements {
    fn default() -> Self {
        let ui_cpu_data = CpuMeasurements::default();
        let ui_memory_data = MemoryMeasurments::default();
        let ui_disk_data = DiskStatMeasurements::default();
        let ui_socket_data = SocketStatMeasurements::default();
        let ui_nvidia_gpu_data = NvidiaGpuMeasurements::default();

        Self {
            ui_cpu_data,
            ui_memory_data,
            ui_disk_data,
            ui_socket_data,
            ui_nvidia_gpu_data,
        }
    }
}
