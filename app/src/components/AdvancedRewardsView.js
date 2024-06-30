import React, { useEffect, useState } from "react";
import { Table, Button } from "antd";

const AdvancedRewardsView = () => {
  const [dataSource, setDataSource] = useState([]);
  const [loading, setLoading] = useState(false); // Added loading state
  const [fetchDataTrigger, setFetchDataTrigger] = useState(false);

  const handleUpdateStatus = async (record) => {
    try {
    } catch (error) {
      console.error("Error updating status:", error);
    } finally {
      setLoading(false); // Set loading state back to false after update
    }
  };

  const columns = [
    {
      title: "User Address",
      dataIndex: "address",
      key: "address",
    },
    {
      title: "Amount (SHIDONK)",
      dataIndex: "shidonk",
      key: "shidonk",
    },
    {
      title: "Amount (SOL)",
      dataIndex: "sol",
      key: "sol",
    },
    {
      title: "Status",
      dataIndex: "status",
      key: "status",
    },
    {
      title: "Update Status",
      key: "update",
      render: (_, record) => (
        <Button
          type="primary"
          className="bg-black"
          onClick={() => handleUpdateStatus(record)}
          disabled={loading} // Disable the button when loading is true
        >
          {loading ? "Updating..." : "Update Status"}
        </Button>
      ),
    },
  ];

  return (
    <div style={{ background: "#1c1c1c", padding: "20px" }}>
      <Table dataSource={dataSource} columns={columns} rowKey="key" size="middle" pagination={false} />
    </div>
  );
};

export default AdvancedRewardsView;
