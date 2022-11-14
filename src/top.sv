module top (
    input wire clk,
    input wire button_s1,
    input wire button_s2,
    output wire [5:0] led_output
);

    logic [5:0] led = 'd0;

    assign led_output = ~led;

    always_ff @ (posedge button_s1) begin
        led <= led + 'd1;
    end
endmodule

`default_nettype wire